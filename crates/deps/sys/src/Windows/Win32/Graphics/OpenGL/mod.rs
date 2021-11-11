#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ChoosePixelFormat();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn DescribePixelFormat();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetEnhMetaFilePixelFormat();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetPixelFormat();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetPixelFormat();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SwapBuffers();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glAccum();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glAlphaFunc();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glAreTexturesResident();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glArrayElement();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glBegin();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glBindTexture();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glBitmap();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glBlendFunc();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glCallList();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glCallLists();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glClear();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glClearAccum();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glClearColor();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glClearDepth();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glClearIndex();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glClearStencil();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glClipPlane();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3b();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3bv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3d();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3dv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3f();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3fv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3i();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3iv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3s();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3sv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3ub();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3ubv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3ui();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3uiv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3us();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor3usv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4b();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4bv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4d();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4dv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4f();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4fv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4i();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4iv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4s();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4sv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4ub();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4ubv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4ui();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4uiv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4us();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColor4usv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColorMask();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColorMaterial();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glColorPointer();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glCopyPixels();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glCopyTexImage1D();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glCopyTexImage2D();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glCopyTexSubImage1D();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glCopyTexSubImage2D();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glCullFace();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glDeleteLists();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glDeleteTextures();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glDepthFunc();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glDepthMask();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glDepthRange();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glDisable();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glDisableClientState();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glDrawArrays();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glDrawBuffer();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glDrawElements();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glDrawPixels();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEdgeFlag();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEdgeFlagPointer();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEdgeFlagv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEnable();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEnableClientState();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEnd();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEndList();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEvalCoord1d();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEvalCoord1dv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEvalCoord1f();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEvalCoord1fv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEvalCoord2d();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEvalCoord2dv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEvalCoord2f();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEvalCoord2fv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEvalMesh1();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEvalMesh2();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEvalPoint1();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glEvalPoint2();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glFeedbackBuffer();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glFinish();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glFlush();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glFogf();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glFogfv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glFogi();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glFogiv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glFrontFace();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glFrustum();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGenLists();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGenTextures();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetBooleanv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetClipPlane();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetDoublev();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetError();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetFloatv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetIntegerv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetLightfv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetLightiv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetMapdv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetMapfv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetMapiv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetMaterialfv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetMaterialiv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetPixelMapfv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetPixelMapuiv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetPixelMapusv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetPointerv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetPolygonStipple();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetString();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetTexEnvfv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetTexEnviv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetTexGendv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetTexGenfv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetTexGeniv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetTexImage();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetTexLevelParameterfv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetTexLevelParameteriv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetTexParameterfv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glGetTexParameteriv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glHint();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIndexMask();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIndexPointer();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIndexd();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIndexdv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIndexf();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIndexfv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIndexi();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIndexiv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIndexs();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIndexsv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIndexub();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIndexubv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glInitNames();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glInterleavedArrays();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIsEnabled();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIsList();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glIsTexture();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLightModelf();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLightModelfv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLightModeli();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLightModeliv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLightf();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLightfv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLighti();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLightiv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLineStipple();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLineWidth();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glListBase();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLoadIdentity();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLoadMatrixd();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLoadMatrixf();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLoadName();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glLogicOp();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMap1d();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMap1f();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMap2d();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMap2f();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMapGrid1d();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMapGrid1f();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMapGrid2d();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMapGrid2f();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMaterialf();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMaterialfv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMateriali();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMaterialiv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMatrixMode();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMultMatrixd();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glMultMatrixf();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glNewList();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glNormal3b();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glNormal3bv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glNormal3d();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glNormal3dv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glNormal3f();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glNormal3fv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glNormal3i();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glNormal3iv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glNormal3s();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glNormal3sv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glNormalPointer();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glOrtho();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPassThrough();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPixelMapfv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPixelMapuiv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPixelMapusv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPixelStoref();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPixelStorei();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPixelTransferf();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPixelTransferi();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPixelZoom();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPointSize();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPolygonMode();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPolygonOffset();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPolygonStipple();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPopAttrib();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPopClientAttrib();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPopMatrix();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPopName();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPrioritizeTextures();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPushAttrib();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPushClientAttrib();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPushMatrix();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glPushName();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos2d();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos2dv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos2f();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos2fv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos2i();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos2iv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos2s();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos2sv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos3d();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos3dv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos3f();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos3fv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos3i();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos3iv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos3s();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos3sv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos4d();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos4dv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos4f();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos4fv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos4i();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos4iv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos4s();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRasterPos4sv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glReadBuffer();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glReadPixels();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRectd();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRectdv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRectf();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRectfv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRecti();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRectiv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRects();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRectsv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRenderMode();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRotated();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glRotatef();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glScaled();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glScalef();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glScissor();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glSelectBuffer();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glShadeModel();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glStencilFunc();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glStencilMask();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glStencilOp();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord1d();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord1dv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord1f();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord1fv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord1i();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord1iv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord1s();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord1sv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord2d();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord2dv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord2f();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord2fv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord2i();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord2iv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord2s();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord2sv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord3d();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord3dv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord3f();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord3fv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord3i();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord3iv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord3s();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord3sv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord4d();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord4dv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord4f();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord4fv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord4i();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord4iv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord4s();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoord4sv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexCoordPointer();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexEnvf();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexEnvfv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexEnvi();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexEnviv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexGend();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexGendv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexGenf();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexGenfv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexGeni();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexGeniv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexImage1D();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexImage2D();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexParameterf();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexParameterfv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexParameteri();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexParameteriv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexSubImage1D();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTexSubImage2D();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTranslated();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glTranslatef();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex2d();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex2dv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex2f();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex2fv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex2i();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex2iv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex2s();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex2sv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex3d();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex3dv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex3f();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex3fv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex3i();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex3iv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex3s();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex3sv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex4d();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex4dv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex4f();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex4fv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex4i();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex4iv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex4s();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertex4sv();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glVertexPointer();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn glViewport();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluBeginCurve();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluBeginPolygon();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluBeginSurface();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluBeginTrim();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluBuild1DMipmaps();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluBuild2DMipmaps();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluCylinder();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluDeleteNurbsRenderer();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluDeleteQuadric();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluDeleteTess();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluDisk();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluEndCurve();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluEndPolygon();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluEndSurface();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluEndTrim();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluErrorString();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn gluErrorUnicodeStringEXT();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluGetNurbsProperty();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluGetString();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluGetTessProperty();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluLoadSamplingMatrices();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluLookAt();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluNewNurbsRenderer();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluNewQuadric();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluNewTess();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluNextContour();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluNurbsCallback();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluNurbsCurve();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluNurbsProperty();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluNurbsSurface();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluOrtho2D();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluPartialDisk();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluPerspective();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluPickMatrix();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluProject();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluPwlCurve();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluQuadricCallback();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluQuadricDrawStyle();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluQuadricNormals();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluQuadricOrientation();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluQuadricTexture();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluScaleImage();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluSphere();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluTessBeginContour();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluTessBeginPolygon();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluTessCallback();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluTessEndContour();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluTessEndPolygon();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluTessNormal();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluTessProperty();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluTessVertex();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn gluUnProject();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn wglCopyContext();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn wglCreateContext();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn wglCreateLayerContext();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn wglDeleteContext();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn wglDescribeLayerPlane();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`*"]
    pub fn wglGetCurrentContext();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn wglGetCurrentDC();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn wglGetLayerPaletteEntries();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn wglGetProcAddress();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn wglMakeCurrent();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn wglRealizeLayerPalette();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn wglSetLayerPaletteEntries();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn wglShareLists();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn wglSwapLayerBuffers();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn wglUseFontBitmapsA();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn wglUseFontBitmapsW();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn wglUseFontOutlinesA();
    #[doc = "*Required features: `Win32_Graphics_OpenGL`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn wglUseFontOutlinesW();
}
