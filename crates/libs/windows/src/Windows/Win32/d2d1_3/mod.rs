#[inline]
pub unsafe fn D2D1GetGradientMeshInteriorPointsFromCoonsPatch(ppoint0: *const windows_numerics::Vector2, ppoint1: *const windows_numerics::Vector2, ppoint2: *const windows_numerics::Vector2, ppoint3: *const windows_numerics::Vector2, ppoint4: *const windows_numerics::Vector2, ppoint5: *const windows_numerics::Vector2, ppoint6: *const windows_numerics::Vector2, ppoint7: *const windows_numerics::Vector2, ppoint8: *const windows_numerics::Vector2, ppoint9: *const windows_numerics::Vector2, ppoint10: *const windows_numerics::Vector2, ppoint11: *const windows_numerics::Vector2, ptensorpoint11: *mut windows_numerics::Vector2, ptensorpoint12: *mut windows_numerics::Vector2, ptensorpoint21: *mut windows_numerics::Vector2, ptensorpoint22: *mut windows_numerics::Vector2) {
    windows_core::link!("d2d1.dll" "system" fn D2D1GetGradientMeshInteriorPointsFromCoonsPatch(ppoint0 : *const windows_numerics::Vector2, ppoint1 : *const windows_numerics::Vector2, ppoint2 : *const windows_numerics::Vector2, ppoint3 : *const windows_numerics::Vector2, ppoint4 : *const windows_numerics::Vector2, ppoint5 : *const windows_numerics::Vector2, ppoint6 : *const windows_numerics::Vector2, ppoint7 : *const windows_numerics::Vector2, ppoint8 : *const windows_numerics::Vector2, ppoint9 : *const windows_numerics::Vector2, ppoint10 : *const windows_numerics::Vector2, ppoint11 : *const windows_numerics::Vector2, ptensorpoint11 : *mut windows_numerics::Vector2, ptensorpoint12 : *mut windows_numerics::Vector2, ptensorpoint21 : *mut windows_numerics::Vector2, ptensorpoint22 : *mut windows_numerics::Vector2));
    unsafe { D2D1GetGradientMeshInteriorPointsFromCoonsPatch(ppoint0, ppoint1, ppoint2, ppoint3, ppoint4, ppoint5, ppoint6, ppoint7, ppoint8, ppoint9, ppoint10, ppoint11, ptensorpoint11 as _, ptensorpoint12 as _, ptensorpoint21 as _, ptensorpoint22 as _) }
}
pub type D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION = i32;
pub const D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_DEFAULT: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION = 0;
pub const D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_DISABLE: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION = 1;
pub const D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_FORCE_DWORD: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION = -1;
pub type D2D1_COLOR_CONTEXT_TYPE = i32;
pub const D2D1_COLOR_CONTEXT_TYPE_DXGI: D2D1_COLOR_CONTEXT_TYPE = 2;
pub const D2D1_COLOR_CONTEXT_TYPE_FORCE_DWORD: D2D1_COLOR_CONTEXT_TYPE = -1;
pub const D2D1_COLOR_CONTEXT_TYPE_ICC: D2D1_COLOR_CONTEXT_TYPE = 0;
pub const D2D1_COLOR_CONTEXT_TYPE_SIMPLE: D2D1_COLOR_CONTEXT_TYPE = 1;
pub type D2D1_GAMMA1 = i32;
pub const D2D1_GAMMA1_FORCE_DWORD: D2D1_GAMMA1 = -1;
pub const D2D1_GAMMA1_G10: D2D1_GAMMA1 = 1;
pub const D2D1_GAMMA1_G2084: D2D1_GAMMA1 = 2;
pub const D2D1_GAMMA1_G22: D2D1_GAMMA1 = 0;
#[repr(C)]
#[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_GRADIENT_MESH_PATCH {
    pub point00: windows_numerics::Vector2,
    pub point01: windows_numerics::Vector2,
    pub point02: windows_numerics::Vector2,
    pub point03: windows_numerics::Vector2,
    pub point10: windows_numerics::Vector2,
    pub point11: windows_numerics::Vector2,
    pub point12: windows_numerics::Vector2,
    pub point13: windows_numerics::Vector2,
    pub point20: windows_numerics::Vector2,
    pub point21: windows_numerics::Vector2,
    pub point22: windows_numerics::Vector2,
    pub point23: windows_numerics::Vector2,
    pub point30: windows_numerics::Vector2,
    pub point31: windows_numerics::Vector2,
    pub point32: windows_numerics::Vector2,
    pub point33: windows_numerics::Vector2,
    pub color00: super::d2dbasetypes::D2D_COLOR_F,
    pub color03: super::d2dbasetypes::D2D_COLOR_F,
    pub color30: super::d2dbasetypes::D2D_COLOR_F,
    pub color33: super::d2dbasetypes::D2D_COLOR_F,
    pub topEdgeMode: D2D1_PATCH_EDGE_MODE,
    pub leftEdgeMode: D2D1_PATCH_EDGE_MODE,
    pub bottomEdgeMode: D2D1_PATCH_EDGE_MODE,
    pub rightEdgeMode: D2D1_PATCH_EDGE_MODE,
}
pub type D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS = u32;
pub const D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_FORCE_DWORD: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS = 4294967295;
pub const D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_LOW_QUALITY_PRIMARY_CONVERSION: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS = 1;
pub const D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_NONE: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS = 0;
pub type D2D1_IMAGE_SOURCE_LOADING_OPTIONS = u32;
pub const D2D1_IMAGE_SOURCE_LOADING_OPTIONS_CACHE_ON_DEMAND: D2D1_IMAGE_SOURCE_LOADING_OPTIONS = 2;
pub const D2D1_IMAGE_SOURCE_LOADING_OPTIONS_FORCE_DWORD: D2D1_IMAGE_SOURCE_LOADING_OPTIONS = 4294967295;
pub const D2D1_IMAGE_SOURCE_LOADING_OPTIONS_NONE: D2D1_IMAGE_SOURCE_LOADING_OPTIONS = 0;
pub const D2D1_IMAGE_SOURCE_LOADING_OPTIONS_RELEASE_SOURCE: D2D1_IMAGE_SOURCE_LOADING_OPTIONS = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_INK_BEZIER_SEGMENT {
    pub point1: D2D1_INK_POINT,
    pub point2: D2D1_INK_POINT,
    pub point3: D2D1_INK_POINT,
}
pub type D2D1_INK_NIB_SHAPE = i32;
pub const D2D1_INK_NIB_SHAPE_FORCE_DWORD: D2D1_INK_NIB_SHAPE = -1;
pub const D2D1_INK_NIB_SHAPE_ROUND: D2D1_INK_NIB_SHAPE = 0;
pub const D2D1_INK_NIB_SHAPE_SQUARE: D2D1_INK_NIB_SHAPE = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_INK_POINT {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_INK_STYLE_PROPERTIES {
    pub nibShape: D2D1_INK_NIB_SHAPE,
    pub nibTransform: windows_numerics::Matrix3x2,
}
pub type D2D1_ORIENTATION = i32;
pub const D2D1_ORIENTATION_DEFAULT: D2D1_ORIENTATION = 1;
pub const D2D1_ORIENTATION_FLIP_HORIZONTAL: D2D1_ORIENTATION = 2;
pub const D2D1_ORIENTATION_FORCE_DWORD: D2D1_ORIENTATION = -1;
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE180: D2D1_ORIENTATION = 3;
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE180_FLIP_HORIZONTAL: D2D1_ORIENTATION = 4;
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE270: D2D1_ORIENTATION = 6;
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE270_FLIP_HORIZONTAL: D2D1_ORIENTATION = 7;
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE90: D2D1_ORIENTATION = 8;
pub const D2D1_ORIENTATION_ROTATE_CLOCKWISE90_FLIP_HORIZONTAL: D2D1_ORIENTATION = 5;
pub type D2D1_PATCH_EDGE_MODE = i32;
pub const D2D1_PATCH_EDGE_MODE_ALIASED: D2D1_PATCH_EDGE_MODE = 0;
pub const D2D1_PATCH_EDGE_MODE_ALIASED_INFLATED: D2D1_PATCH_EDGE_MODE = 2;
pub const D2D1_PATCH_EDGE_MODE_ANTIALIASED: D2D1_PATCH_EDGE_MODE = 1;
pub const D2D1_PATCH_EDGE_MODE_FORCE_DWORD: D2D1_PATCH_EDGE_MODE = -1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_SIMPLE_COLOR_PROFILE {
    pub redPrimary: windows_numerics::Vector2,
    pub greenPrimary: windows_numerics::Vector2,
    pub bluePrimary: windows_numerics::Vector2,
    pub whitePointXZ: windows_numerics::Vector2,
    pub gamma: D2D1_GAMMA1,
}
pub type D2D1_SPRITE_OPTIONS = u32;
pub const D2D1_SPRITE_OPTIONS_CLAMP_TO_SOURCE_RECTANGLE: D2D1_SPRITE_OPTIONS = 1;
pub const D2D1_SPRITE_OPTIONS_FORCE_DWORD: D2D1_SPRITE_OPTIONS = 4294967295;
pub const D2D1_SPRITE_OPTIONS_NONE: D2D1_SPRITE_OPTIONS = 0;
pub type D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS = u32;
pub const D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_DISABLE_DPI_SCALE: D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS = 1;
pub const D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_FORCE_DWORD: D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS = 4294967295;
pub const D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_NONE: D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS = 0;
#[repr(C)]
#[cfg(feature = "Win32_d2d1_1")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES {
    pub orientation: D2D1_ORIENTATION,
    pub scaleX: f32,
    pub scaleY: f32,
    pub interpolationMode: super::d2d1_1::D2D1_INTERPOLATION_MODE,
    pub options: D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
windows_core::imp::define_interface!(ID2D1ColorContext1, ID2D1ColorContext1_Vtbl, 0x1ab42875_c57f_4be9_bd85_9cd78d6f55ee);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
impl core::ops::Deref for ID2D1ColorContext1 {
    type Target = super::d2d1_1::ID2D1ColorContext;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
windows_core::imp::interface_hierarchy!(ID2D1ColorContext1, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1_1::ID2D1ColorContext);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
impl ID2D1ColorContext1 {
    pub unsafe fn GetColorContextType(&self) -> D2D1_COLOR_CONTEXT_TYPE {
        unsafe { (windows_core::Interface::vtable(self).GetColorContextType)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_dxgicommon")]
    pub unsafe fn GetDXGIColorSpace(&self) -> super::dxgicommon::DXGI_COLOR_SPACE_TYPE {
        unsafe { (windows_core::Interface::vtable(self).GetDXGIColorSpace)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetSimpleColorProfile(&self, simpleprofile: *mut D2D1_SIMPLE_COLOR_PROFILE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSimpleColorProfile)(windows_core::Interface::as_raw(self), simpleprofile as _) }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1ColorContext1_Vtbl {
    pub base__: super::d2d1_1::ID2D1ColorContext_Vtbl,
    pub GetColorContextType: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_COLOR_CONTEXT_TYPE,
    #[cfg(feature = "Win32_dxgicommon")]
    pub GetDXGIColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::dxgicommon::DXGI_COLOR_SPACE_TYPE,
    #[cfg(not(feature = "Win32_dxgicommon"))]
    GetDXGIColorSpace: usize,
    pub GetSimpleColorProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_SIMPLE_COLOR_PROFILE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_dxgicommon"))]
pub trait ID2D1ColorContext1_Impl: super::d2d1_1::ID2D1ColorContext_Impl {
    fn GetColorContextType(&self) -> D2D1_COLOR_CONTEXT_TYPE;
    fn GetDXGIColorSpace(&self) -> super::dxgicommon::DXGI_COLOR_SPACE_TYPE;
    fn GetSimpleColorProfile(&self, simpleprofile: *mut D2D1_SIMPLE_COLOR_PROFILE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_dxgicommon"))]
impl ID2D1ColorContext1_Vtbl {
    pub const fn new<Identity: ID2D1ColorContext1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetColorContextType<Identity: ID2D1ColorContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_COLOR_CONTEXT_TYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1ColorContext1_Impl::GetColorContextType(this)
            }
        }
        unsafe extern "system" fn GetDXGIColorSpace<Identity: ID2D1ColorContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::dxgicommon::DXGI_COLOR_SPACE_TYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1ColorContext1_Impl::GetDXGIColorSpace(this)
            }
        }
        unsafe extern "system" fn GetSimpleColorProfile<Identity: ID2D1ColorContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, simpleprofile: *mut D2D1_SIMPLE_COLOR_PROFILE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1ColorContext1_Impl::GetSimpleColorProfile(this, core::mem::transmute_copy(&simpleprofile)).into()
            }
        }
        Self {
            base__: super::d2d1_1::ID2D1ColorContext_Vtbl::new::<Identity, OFFSET>(),
            GetColorContextType: GetColorContextType::<Identity, OFFSET>,
            GetDXGIColorSpace: GetDXGIColorSpace::<Identity, OFFSET>,
            GetSimpleColorProfile: GetSimpleColorProfile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1ColorContext1 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1ColorContext as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_dxgicommon"))]
impl windows_core::RuntimeName for ID2D1ColorContext1 {}
#[cfg(all(feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::define_interface!(ID2D1CommandSink2, ID2D1CommandSink2_Vtbl, 0x3bab440e_417e_47df_a2e2_bc0be6a00916);
#[cfg(all(feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl core::ops::Deref for ID2D1CommandSink2 {
    type Target = super::d2d1_2::ID2D1CommandSink1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::interface_hierarchy!(ID2D1CommandSink2, windows_core::IUnknown, super::d2d1_1::ID2D1CommandSink, super::d2d1_2::ID2D1CommandSink1);
#[cfg(all(feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl ID2D1CommandSink2 {
    #[cfg(feature = "Win32_d2d1")]
    pub unsafe fn DrawInk<P0, P1, P2>(&self, ink: P0, brush: P1, inkstyle: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID2D1Ink>,
        P1: windows_core::Param<super::d2d1::ID2D1Brush>,
        P2: windows_core::Param<ID2D1InkStyle>,
    {
        unsafe { (windows_core::Interface::vtable(self).DrawInk)(windows_core::Interface::as_raw(self), ink.param().abi(), brush.param().abi(), inkstyle.param().abi()) }
    }
    #[cfg(feature = "Win32_d2d1")]
    pub unsafe fn DrawGradientMesh<P0>(&self, gradientmesh: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID2D1GradientMesh>,
    {
        unsafe { (windows_core::Interface::vtable(self).DrawGradientMesh)(windows_core::Interface::as_raw(self), gradientmesh.param().abi()) }
    }
    #[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
    pub unsafe fn DrawGdiMetafile<P0>(&self, gdimetafile: P0, destinationrectangle: Option<*const super::dcommon::D2D_RECT_F>, sourcerectangle: Option<*const super::dcommon::D2D_RECT_F>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d2d1_1::ID2D1GdiMetafile>,
    {
        unsafe { (windows_core::Interface::vtable(self).DrawGdiMetafile)(windows_core::Interface::as_raw(self), gdimetafile.param().abi(), destinationrectangle.unwrap_or(core::mem::zeroed()) as _, sourcerectangle.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[cfg(all(feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1CommandSink2_Vtbl {
    pub base__: super::d2d1_2::ID2D1CommandSink1_Vtbl,
    #[cfg(feature = "Win32_d2d1")]
    pub DrawInk: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d2d1"))]
    DrawInk: usize,
    #[cfg(feature = "Win32_d2d1")]
    pub DrawGradientMesh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d2d1"))]
    DrawGradientMesh: usize,
    #[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
    pub DrawGdiMetafile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::dcommon::D2D_RECT_F, *const super::dcommon::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d2d1", feature = "Win32_dcommon")))]
    DrawGdiMetafile: usize,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
pub trait ID2D1CommandSink2_Impl: super::d2d1_2::ID2D1CommandSink1_Impl {
    fn DrawInk(&self, ink: windows_core::Ref<ID2D1Ink>, brush: windows_core::Ref<super::d2d1::ID2D1Brush>, inkstyle: windows_core::Ref<ID2D1InkStyle>) -> windows_core::Result<()>;
    fn DrawGradientMesh(&self, gradientmesh: windows_core::Ref<ID2D1GradientMesh>) -> windows_core::Result<()>;
    fn DrawGdiMetafile(&self, gdimetafile: windows_core::Ref<super::d2d1_1::ID2D1GdiMetafile>, destinationrectangle: *const super::dcommon::D2D_RECT_F, sourcerectangle: *const super::dcommon::D2D_RECT_F) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
impl ID2D1CommandSink2_Vtbl {
    pub const fn new<Identity: ID2D1CommandSink2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DrawInk<Identity: ID2D1CommandSink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ink: *mut core::ffi::c_void, brush: *mut core::ffi::c_void, inkstyle: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink2_Impl::DrawInk(this, core::mem::transmute_copy(&ink), core::mem::transmute_copy(&brush), core::mem::transmute_copy(&inkstyle)).into()
            }
        }
        unsafe extern "system" fn DrawGradientMesh<Identity: ID2D1CommandSink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradientmesh: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink2_Impl::DrawGradientMesh(this, core::mem::transmute_copy(&gradientmesh)).into()
            }
        }
        unsafe extern "system" fn DrawGdiMetafile<Identity: ID2D1CommandSink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gdimetafile: *mut core::ffi::c_void, destinationrectangle: *const super::dcommon::D2D_RECT_F, sourcerectangle: *const super::dcommon::D2D_RECT_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink2_Impl::DrawGdiMetafile(this, core::mem::transmute_copy(&gdimetafile), core::mem::transmute_copy(&destinationrectangle), core::mem::transmute_copy(&sourcerectangle)).into()
            }
        }
        Self {
            base__: super::d2d1_2::ID2D1CommandSink1_Vtbl::new::<Identity, OFFSET>(),
            DrawInk: DrawInk::<Identity, OFFSET>,
            DrawGradientMesh: DrawGradientMesh::<Identity, OFFSET>,
            DrawGdiMetafile: DrawGdiMetafile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1CommandSink2 as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1CommandSink as windows_core::Interface>::IID || iid == &<super::d2d1_2::ID2D1CommandSink1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
impl windows_core::RuntimeName for ID2D1CommandSink2 {}
#[cfg(all(feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::define_interface!(ID2D1CommandSink3, ID2D1CommandSink3_Vtbl, 0x18079135_4cf3_4868_bc8e_06067e6d242d);
#[cfg(all(feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl core::ops::Deref for ID2D1CommandSink3 {
    type Target = ID2D1CommandSink2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::interface_hierarchy!(ID2D1CommandSink3, windows_core::IUnknown, super::d2d1_1::ID2D1CommandSink, super::d2d1_2::ID2D1CommandSink1, ID2D1CommandSink2);
#[cfg(all(feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl ID2D1CommandSink3 {
    #[cfg(feature = "Win32_d2d1")]
    pub unsafe fn DrawSpriteBatch<P0, P3>(&self, spritebatch: P0, startindex: u32, spritecount: u32, bitmap: P3, interpolationmode: super::d2d1::D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID2D1SpriteBatch>,
        P3: windows_core::Param<super::d2d1::ID2D1Bitmap>,
    {
        unsafe { (windows_core::Interface::vtable(self).DrawSpriteBatch)(windows_core::Interface::as_raw(self), spritebatch.param().abi(), startindex, spritecount, bitmap.param().abi(), interpolationmode, spriteoptions) }
    }
}
#[cfg(all(feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1CommandSink3_Vtbl {
    pub base__: ID2D1CommandSink2_Vtbl,
    #[cfg(feature = "Win32_d2d1")]
    pub DrawSpriteBatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, super::d2d1::D2D1_BITMAP_INTERPOLATION_MODE, D2D1_SPRITE_OPTIONS) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d2d1"))]
    DrawSpriteBatch: usize,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
pub trait ID2D1CommandSink3_Impl: ID2D1CommandSink2_Impl {
    fn DrawSpriteBatch(&self, spritebatch: windows_core::Ref<ID2D1SpriteBatch>, startindex: u32, spritecount: u32, bitmap: windows_core::Ref<super::d2d1::ID2D1Bitmap>, interpolationmode: super::d2d1::D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
impl ID2D1CommandSink3_Vtbl {
    pub const fn new<Identity: ID2D1CommandSink3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DrawSpriteBatch<Identity: ID2D1CommandSink3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, spritebatch: *mut core::ffi::c_void, startindex: u32, spritecount: u32, bitmap: *mut core::ffi::c_void, interpolationmode: super::d2d1::D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink3_Impl::DrawSpriteBatch(this, core::mem::transmute_copy(&spritebatch), core::mem::transmute_copy(&startindex), core::mem::transmute_copy(&spritecount), core::mem::transmute_copy(&bitmap), core::mem::transmute_copy(&interpolationmode), core::mem::transmute_copy(&spriteoptions)).into()
            }
        }
        Self { base__: ID2D1CommandSink2_Vtbl::new::<Identity, OFFSET>(), DrawSpriteBatch: DrawSpriteBatch::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1CommandSink3 as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1CommandSink as windows_core::Interface>::IID || iid == &<super::d2d1_2::ID2D1CommandSink1 as windows_core::Interface>::IID || iid == &<ID2D1CommandSink2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
impl windows_core::RuntimeName for ID2D1CommandSink3 {}
#[cfg(all(feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::define_interface!(ID2D1CommandSink4, ID2D1CommandSink4_Vtbl, 0xc78a6519_40d6_4218_b2de_beeeb744bb3e);
#[cfg(all(feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl core::ops::Deref for ID2D1CommandSink4 {
    type Target = ID2D1CommandSink3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::interface_hierarchy!(ID2D1CommandSink4, windows_core::IUnknown, super::d2d1_1::ID2D1CommandSink, super::d2d1_2::ID2D1CommandSink1, ID2D1CommandSink2, ID2D1CommandSink3);
#[cfg(all(feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl ID2D1CommandSink4 {
    pub unsafe fn SetPrimitiveBlend2(&self, primitiveblend: super::d2d1_1::D2D1_PRIMITIVE_BLEND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrimitiveBlend2)(windows_core::Interface::as_raw(self), primitiveblend) }
    }
}
#[cfg(all(feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1CommandSink4_Vtbl {
    pub base__: ID2D1CommandSink3_Vtbl,
    pub SetPrimitiveBlend2: unsafe extern "system" fn(*mut core::ffi::c_void, super::d2d1_1::D2D1_PRIMITIVE_BLEND) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
pub trait ID2D1CommandSink4_Impl: ID2D1CommandSink3_Impl {
    fn SetPrimitiveBlend2(&self, primitiveblend: super::d2d1_1::D2D1_PRIMITIVE_BLEND) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
impl ID2D1CommandSink4_Vtbl {
    pub const fn new<Identity: ID2D1CommandSink4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPrimitiveBlend2<Identity: ID2D1CommandSink4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, primitiveblend: super::d2d1_1::D2D1_PRIMITIVE_BLEND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink4_Impl::SetPrimitiveBlend2(this, core::mem::transmute_copy(&primitiveblend)).into()
            }
        }
        Self { base__: ID2D1CommandSink3_Vtbl::new::<Identity, OFFSET>(), SetPrimitiveBlend2: SetPrimitiveBlend2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1CommandSink4 as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1CommandSink as windows_core::Interface>::IID || iid == &<super::d2d1_2::ID2D1CommandSink1 as windows_core::Interface>::IID || iid == &<ID2D1CommandSink2 as windows_core::Interface>::IID || iid == &<ID2D1CommandSink3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
impl windows_core::RuntimeName for ID2D1CommandSink4 {}
#[cfg(all(feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::define_interface!(ID2D1CommandSink5, ID2D1CommandSink5_Vtbl, 0x7047dd26_b1e7_44a7_959a_8349e2144fa8);
#[cfg(all(feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl core::ops::Deref for ID2D1CommandSink5 {
    type Target = ID2D1CommandSink4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::interface_hierarchy!(ID2D1CommandSink5, windows_core::IUnknown, super::d2d1_1::ID2D1CommandSink, super::d2d1_2::ID2D1CommandSink1, ID2D1CommandSink2, ID2D1CommandSink3, ID2D1CommandSink4);
#[cfg(all(feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl ID2D1CommandSink5 {
    #[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1effects", feature = "Win32_dcommon"))]
    pub unsafe fn BlendImage<P0>(&self, image: P0, blendmode: super::d2d1effects::D2D1_BLEND_MODE, targetoffset: Option<*const windows_numerics::Vector2>, imagerectangle: Option<*const super::dcommon::D2D_RECT_F>, interpolationmode: super::d2d1_1::D2D1_INTERPOLATION_MODE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d2d1::ID2D1Image>,
    {
        unsafe { (windows_core::Interface::vtable(self).BlendImage)(windows_core::Interface::as_raw(self), image.param().abi(), blendmode, targetoffset.unwrap_or(core::mem::zeroed()) as _, imagerectangle.unwrap_or(core::mem::zeroed()) as _, interpolationmode) }
    }
}
#[cfg(all(feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1CommandSink5_Vtbl {
    pub base__: ID2D1CommandSink4_Vtbl,
    #[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1effects", feature = "Win32_dcommon"))]
    pub BlendImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::d2d1effects::D2D1_BLEND_MODE, *const windows_numerics::Vector2, *const super::dcommon::D2D_RECT_F, super::d2d1_1::D2D1_INTERPOLATION_MODE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d2d1", feature = "Win32_d2d1effects", feature = "Win32_dcommon")))]
    BlendImage: usize,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2d1effects", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
pub trait ID2D1CommandSink5_Impl: ID2D1CommandSink4_Impl {
    fn BlendImage(&self, image: windows_core::Ref<super::d2d1::ID2D1Image>, blendmode: super::d2d1effects::D2D1_BLEND_MODE, targetoffset: *const windows_numerics::Vector2, imagerectangle: *const super::dcommon::D2D_RECT_F, interpolationmode: super::d2d1_1::D2D1_INTERPOLATION_MODE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2d1effects", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
impl ID2D1CommandSink5_Vtbl {
    pub const fn new<Identity: ID2D1CommandSink5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BlendImage<Identity: ID2D1CommandSink5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, image: *mut core::ffi::c_void, blendmode: super::d2d1effects::D2D1_BLEND_MODE, targetoffset: *const windows_numerics::Vector2, imagerectangle: *const super::dcommon::D2D_RECT_F, interpolationmode: super::d2d1_1::D2D1_INTERPOLATION_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink5_Impl::BlendImage(this, core::mem::transmute_copy(&image), core::mem::transmute_copy(&blendmode), core::mem::transmute_copy(&targetoffset), core::mem::transmute_copy(&imagerectangle), core::mem::transmute_copy(&interpolationmode)).into()
            }
        }
        Self { base__: ID2D1CommandSink4_Vtbl::new::<Identity, OFFSET>(), BlendImage: BlendImage::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1CommandSink5 as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1CommandSink as windows_core::Interface>::IID || iid == &<super::d2d1_2::ID2D1CommandSink1 as windows_core::Interface>::IID || iid == &<ID2D1CommandSink2 as windows_core::Interface>::IID || iid == &<ID2D1CommandSink3 as windows_core::Interface>::IID || iid == &<ID2D1CommandSink4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2d1effects", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
impl windows_core::RuntimeName for ID2D1CommandSink5 {}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::define_interface!(ID2D1Device2, ID2D1Device2_Vtbl, 0xa44472e1_8dfb_4e60_8492_6e2861c9ca8b);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl core::ops::Deref for ID2D1Device2 {
    type Target = super::d2d1_2::ID2D1Device1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::interface_hierarchy!(ID2D1Device2, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1_1::ID2D1Device, super::d2d1_2::ID2D1Device1);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl ID2D1Device2 {
    pub unsafe fn CreateDeviceContext(&self, options: super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDeviceContext)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FlushDeviceContexts<P0>(&self, bitmap: P0)
    where
        P0: windows_core::Param<super::d2d1::ID2D1Bitmap>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FlushDeviceContexts)(windows_core::Interface::as_raw(self), bitmap.param().abi());
        }
    }
    #[cfg(feature = "Win32_dxgi")]
    pub unsafe fn GetDxgiDevice(&self) -> windows_core::Result<super::dxgi::IDXGIDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDxgiDevice)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Device2_Vtbl {
    pub base__: super::d2d1_2::ID2D1Device1_Vtbl,
    pub CreateDeviceContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FlushDeviceContexts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(feature = "Win32_dxgi")]
    pub GetDxgiDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgi"))]
    GetDxgiDevice: usize,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dxgi", feature = "Win32_wincodec"))]
pub trait ID2D1Device2_Impl: super::d2d1_2::ID2D1Device1_Impl {
    fn CreateDeviceContext(&self, options: super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext2>;
    fn FlushDeviceContexts(&self, bitmap: windows_core::Ref<super::d2d1::ID2D1Bitmap>);
    fn GetDxgiDevice(&self) -> windows_core::Result<super::dxgi::IDXGIDevice>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dxgi", feature = "Win32_wincodec"))]
impl ID2D1Device2_Vtbl {
    pub const fn new<Identity: ID2D1Device2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDeviceContext<Identity: ID2D1Device2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Device2_Impl::CreateDeviceContext(this, core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        devicecontext2.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FlushDeviceContexts<Identity: ID2D1Device2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitmap: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Device2_Impl::FlushDeviceContexts(this, core::mem::transmute_copy(&bitmap));
            }
        }
        unsafe extern "system" fn GetDxgiDevice<Identity: ID2D1Device2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxgidevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Device2_Impl::GetDxgiDevice(this) {
                    Ok(ok__) => {
                        dxgidevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::d2d1_2::ID2D1Device1_Vtbl::new::<Identity, OFFSET>(),
            CreateDeviceContext: CreateDeviceContext::<Identity, OFFSET>,
            FlushDeviceContexts: FlushDeviceContexts::<Identity, OFFSET>,
            GetDxgiDevice: GetDxgiDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Device2 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1Device as windows_core::Interface>::IID || iid == &<super::d2d1_2::ID2D1Device1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dxgi", feature = "Win32_wincodec"))]
impl windows_core::RuntimeName for ID2D1Device2 {}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::define_interface!(ID2D1Device3, ID2D1Device3_Vtbl, 0x852f2087_802c_4037_ab60_ff2e7ee6fc01);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl core::ops::Deref for ID2D1Device3 {
    type Target = ID2D1Device2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::interface_hierarchy!(ID2D1Device3, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1_1::ID2D1Device, super::d2d1_2::ID2D1Device1, ID2D1Device2);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl ID2D1Device3 {
    pub unsafe fn CreateDeviceContext(&self, options: super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDeviceContext)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Device3_Vtbl {
    pub base__: ID2D1Device2_Vtbl,
    pub CreateDeviceContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dxgi", feature = "Win32_wincodec"))]
pub trait ID2D1Device3_Impl: ID2D1Device2_Impl {
    fn CreateDeviceContext(&self, options: super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext3>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dxgi", feature = "Win32_wincodec"))]
impl ID2D1Device3_Vtbl {
    pub const fn new<Identity: ID2D1Device3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDeviceContext<Identity: ID2D1Device3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext3: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Device3_Impl::CreateDeviceContext(this, core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        devicecontext3.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ID2D1Device2_Vtbl::new::<Identity, OFFSET>(), CreateDeviceContext: CreateDeviceContext::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Device3 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1Device as windows_core::Interface>::IID || iid == &<super::d2d1_2::ID2D1Device1 as windows_core::Interface>::IID || iid == &<ID2D1Device2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dxgi", feature = "Win32_wincodec"))]
impl windows_core::RuntimeName for ID2D1Device3 {}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::define_interface!(ID2D1Device4, ID2D1Device4_Vtbl, 0xd7bdb159_5683_4a46_bc9c_72dc720b858b);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl core::ops::Deref for ID2D1Device4 {
    type Target = ID2D1Device3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::interface_hierarchy!(ID2D1Device4, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1_1::ID2D1Device, super::d2d1_2::ID2D1Device1, ID2D1Device2, ID2D1Device3);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl ID2D1Device4 {
    pub unsafe fn CreateDeviceContext(&self, options: super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDeviceContext)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetMaximumColorGlyphCacheMemory(&self, maximuminbytes: u64) {
        unsafe {
            (windows_core::Interface::vtable(self).SetMaximumColorGlyphCacheMemory)(windows_core::Interface::as_raw(self), maximuminbytes);
        }
    }
    pub unsafe fn GetMaximumColorGlyphCacheMemory(&self) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetMaximumColorGlyphCacheMemory)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Device4_Vtbl {
    pub base__: ID2D1Device3_Vtbl,
    pub CreateDeviceContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMaximumColorGlyphCacheMemory: unsafe extern "system" fn(*mut core::ffi::c_void, u64),
    pub GetMaximumColorGlyphCacheMemory: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dxgi", feature = "Win32_wincodec"))]
pub trait ID2D1Device4_Impl: ID2D1Device3_Impl {
    fn CreateDeviceContext(&self, options: super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext4>;
    fn SetMaximumColorGlyphCacheMemory(&self, maximuminbytes: u64);
    fn GetMaximumColorGlyphCacheMemory(&self) -> u64;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dxgi", feature = "Win32_wincodec"))]
impl ID2D1Device4_Vtbl {
    pub const fn new<Identity: ID2D1Device4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDeviceContext<Identity: ID2D1Device4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext4: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Device4_Impl::CreateDeviceContext(this, core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        devicecontext4.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaximumColorGlyphCacheMemory<Identity: ID2D1Device4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maximuminbytes: u64) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Device4_Impl::SetMaximumColorGlyphCacheMemory(this, core::mem::transmute_copy(&maximuminbytes));
            }
        }
        unsafe extern "system" fn GetMaximumColorGlyphCacheMemory<Identity: ID2D1Device4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Device4_Impl::GetMaximumColorGlyphCacheMemory(this)
            }
        }
        Self {
            base__: ID2D1Device3_Vtbl::new::<Identity, OFFSET>(),
            CreateDeviceContext: CreateDeviceContext::<Identity, OFFSET>,
            SetMaximumColorGlyphCacheMemory: SetMaximumColorGlyphCacheMemory::<Identity, OFFSET>,
            GetMaximumColorGlyphCacheMemory: GetMaximumColorGlyphCacheMemory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Device4 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1Device as windows_core::Interface>::IID || iid == &<super::d2d1_2::ID2D1Device1 as windows_core::Interface>::IID || iid == &<ID2D1Device2 as windows_core::Interface>::IID || iid == &<ID2D1Device3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dxgi", feature = "Win32_wincodec"))]
impl windows_core::RuntimeName for ID2D1Device4 {}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::define_interface!(ID2D1Device5, ID2D1Device5_Vtbl, 0xd55ba0a4_6405_4694_aef5_08ee1a4358b4);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl core::ops::Deref for ID2D1Device5 {
    type Target = ID2D1Device4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::interface_hierarchy!(ID2D1Device5, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1_1::ID2D1Device, super::d2d1_2::ID2D1Device1, ID2D1Device2, ID2D1Device3, ID2D1Device4);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl ID2D1Device5 {
    pub unsafe fn CreateDeviceContext(&self, options: super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext5> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDeviceContext)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Device5_Vtbl {
    pub base__: ID2D1Device4_Vtbl,
    pub CreateDeviceContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dxgi", feature = "Win32_wincodec"))]
pub trait ID2D1Device5_Impl: ID2D1Device4_Impl {
    fn CreateDeviceContext(&self, options: super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext5>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dxgi", feature = "Win32_wincodec"))]
impl ID2D1Device5_Vtbl {
    pub const fn new<Identity: ID2D1Device5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDeviceContext<Identity: ID2D1Device5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext5: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Device5_Impl::CreateDeviceContext(this, core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        devicecontext5.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ID2D1Device4_Vtbl::new::<Identity, OFFSET>(), CreateDeviceContext: CreateDeviceContext::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Device5 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1Device as windows_core::Interface>::IID || iid == &<super::d2d1_2::ID2D1Device1 as windows_core::Interface>::IID || iid == &<ID2D1Device2 as windows_core::Interface>::IID || iid == &<ID2D1Device3 as windows_core::Interface>::IID || iid == &<ID2D1Device4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dxgi", feature = "Win32_wincodec"))]
impl windows_core::RuntimeName for ID2D1Device5 {}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::define_interface!(ID2D1Device6, ID2D1Device6_Vtbl, 0x7bfef914_2d75_4bad_be87_e18ddb077b6d);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl core::ops::Deref for ID2D1Device6 {
    type Target = ID2D1Device5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::interface_hierarchy!(ID2D1Device6, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1_1::ID2D1Device, super::d2d1_2::ID2D1Device1, ID2D1Device2, ID2D1Device3, ID2D1Device4, ID2D1Device5);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl ID2D1Device6 {
    pub unsafe fn CreateDeviceContext(&self, options: super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext6> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDeviceContext)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Device6_Vtbl {
    pub base__: ID2D1Device5_Vtbl,
    pub CreateDeviceContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dxgi", feature = "Win32_wincodec"))]
pub trait ID2D1Device6_Impl: ID2D1Device5_Impl {
    fn CreateDeviceContext(&self, options: super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext6>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dxgi", feature = "Win32_wincodec"))]
impl ID2D1Device6_Vtbl {
    pub const fn new<Identity: ID2D1Device6_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDeviceContext<Identity: ID2D1Device6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext6: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Device6_Impl::CreateDeviceContext(this, core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        devicecontext6.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ID2D1Device5_Vtbl::new::<Identity, OFFSET>(), CreateDeviceContext: CreateDeviceContext::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Device6 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1Device as windows_core::Interface>::IID || iid == &<super::d2d1_2::ID2D1Device1 as windows_core::Interface>::IID || iid == &<ID2D1Device2 as windows_core::Interface>::IID || iid == &<ID2D1Device3 as windows_core::Interface>::IID || iid == &<ID2D1Device4 as windows_core::Interface>::IID || iid == &<ID2D1Device5 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dxgi", feature = "Win32_wincodec"))]
impl windows_core::RuntimeName for ID2D1Device6 {}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::define_interface!(ID2D1Device7, ID2D1Device7_Vtbl, 0xf07c8968_dd4e_4ba6_9cbd_eb6d3752dcbb);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl core::ops::Deref for ID2D1Device7 {
    type Target = ID2D1Device6;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::interface_hierarchy!(ID2D1Device7, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1_1::ID2D1Device, super::d2d1_2::ID2D1Device1, ID2D1Device2, ID2D1Device3, ID2D1Device4, ID2D1Device5, ID2D1Device6);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl ID2D1Device7 {
    pub unsafe fn CreateDeviceContext(&self, options: super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext7> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDeviceContext)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Device7_Vtbl {
    pub base__: ID2D1Device6_Vtbl,
    pub CreateDeviceContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dxgi", feature = "Win32_wincodec"))]
pub trait ID2D1Device7_Impl: ID2D1Device6_Impl {
    fn CreateDeviceContext(&self, options: super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext7>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dxgi", feature = "Win32_wincodec"))]
impl ID2D1Device7_Vtbl {
    pub const fn new<Identity: ID2D1Device7_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDeviceContext<Identity: ID2D1Device7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Device7_Impl::CreateDeviceContext(this, core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        devicecontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ID2D1Device6_Vtbl::new::<Identity, OFFSET>(), CreateDeviceContext: CreateDeviceContext::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Device7 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1Device as windows_core::Interface>::IID || iid == &<super::d2d1_2::ID2D1Device1 as windows_core::Interface>::IID || iid == &<ID2D1Device2 as windows_core::Interface>::IID || iid == &<ID2D1Device3 as windows_core::Interface>::IID || iid == &<ID2D1Device4 as windows_core::Interface>::IID || iid == &<ID2D1Device5 as windows_core::Interface>::IID || iid == &<ID2D1Device6 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dxgi", feature = "Win32_wincodec"))]
impl windows_core::RuntimeName for ID2D1Device7 {}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::define_interface!(ID2D1DeviceContext2, ID2D1DeviceContext2_Vtbl, 0x394ea6a3_0c34_4321_950b_6ca20f0be6c7);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl core::ops::Deref for ID2D1DeviceContext2 {
    type Target = super::d2d1_2::ID2D1DeviceContext1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::interface_hierarchy!(ID2D1DeviceContext2, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1::ID2D1RenderTarget, super::d2d1_1::ID2D1DeviceContext, super::d2d1_2::ID2D1DeviceContext1);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl ID2D1DeviceContext2 {
    pub unsafe fn CreateInk(&self, startpoint: *const D2D1_INK_POINT) -> windows_core::Result<ID2D1Ink> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInk)(windows_core::Interface::as_raw(self), startpoint, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateInkStyle(&self, inkstyleproperties: Option<*const D2D1_INK_STYLE_PROPERTIES>) -> windows_core::Result<ID2D1InkStyle> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInkStyle)(windows_core::Interface::as_raw(self), inkstyleproperties.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub unsafe fn CreateGradientMesh(&self, patches: &[D2D1_GRADIENT_MESH_PATCH]) -> windows_core::Result<ID2D1GradientMesh> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGradientMesh)(windows_core::Interface::as_raw(self), core::mem::transmute(patches.as_ptr()), patches.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_wincodec"))]
    pub unsafe fn CreateImageSourceFromWic<P0>(&self, wicbitmapsource: P0, loadingoptions: D2D1_IMAGE_SOURCE_LOADING_OPTIONS, alphamode: super::dcommon::D2D1_ALPHA_MODE) -> windows_core::Result<ID2D1ImageSourceFromWic>
    where
        P0: windows_core::Param<super::wincodec::IWICBitmapSource>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateImageSourceFromWic)(windows_core::Interface::as_raw(self), wicbitmapsource.param().abi(), loadingoptions, alphamode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateLookupTable3D(&self, precision: super::d2d1_1::D2D1_BUFFER_PRECISION, extents: &[u32; 3], data: &[u8], strides: &[u32; 2]) -> windows_core::Result<ID2D1LookupTable3D> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateLookupTable3D)(windows_core::Interface::as_raw(self), precision, core::mem::transmute(extents.as_ptr()), core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap(), core::mem::transmute(strides.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgicommon"))]
    pub unsafe fn CreateImageSourceFromDxgi(&self, surfaces: &[Option<super::dxgi::IDXGISurface>], colorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE, options: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS) -> windows_core::Result<ID2D1ImageSource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateImageSourceFromDxgi)(windows_core::Interface::as_raw(self), core::mem::transmute(surfaces.as_ptr()), surfaces.len().try_into().unwrap(), colorspace, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetGradientMeshWorldBounds<P0>(&self, gradientmesh: P0) -> windows_core::Result<super::dcommon::D2D_RECT_F>
    where
        P0: windows_core::Param<ID2D1GradientMesh>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGradientMeshWorldBounds)(windows_core::Interface::as_raw(self), gradientmesh.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DrawInk<P0, P1, P2>(&self, ink: P0, brush: P1, inkstyle: P2)
    where
        P0: windows_core::Param<ID2D1Ink>,
        P1: windows_core::Param<super::d2d1::ID2D1Brush>,
        P2: windows_core::Param<ID2D1InkStyle>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawInk)(windows_core::Interface::as_raw(self), ink.param().abi(), brush.param().abi(), inkstyle.param().abi());
        }
    }
    pub unsafe fn DrawGradientMesh<P0>(&self, gradientmesh: P0)
    where
        P0: windows_core::Param<ID2D1GradientMesh>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawGradientMesh)(windows_core::Interface::as_raw(self), gradientmesh.param().abi());
        }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn DrawGdiMetafile<P0>(&self, gdimetafile: P0, destinationrectangle: Option<*const super::dcommon::D2D_RECT_F>, sourcerectangle: Option<*const super::dcommon::D2D_RECT_F>)
    where
        P0: windows_core::Param<super::d2d1_1::ID2D1GdiMetafile>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawGdiMetafile)(windows_core::Interface::as_raw(self), gdimetafile.param().abi(), destinationrectangle.unwrap_or(core::mem::zeroed()) as _, sourcerectangle.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn CreateTransformedImageSource<P0>(&self, imagesource: P0, properties: *const D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES) -> windows_core::Result<ID2D1TransformedImageSource>
    where
        P0: windows_core::Param<ID2D1ImageSource>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTransformedImageSource)(windows_core::Interface::as_raw(self), imagesource.param().abi(), properties, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1DeviceContext2_Vtbl {
    pub base__: super::d2d1_2::ID2D1DeviceContext1_Vtbl,
    pub CreateInk: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_INK_POINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInkStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_INK_STYLE_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub CreateGradientMesh: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_GRADIENT_MESH_PATCH, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype")))]
    CreateGradientMesh: usize,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_wincodec"))]
    pub CreateImageSourceFromWic: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, D2D1_IMAGE_SOURCE_LOADING_OPTIONS, super::dcommon::D2D1_ALPHA_MODE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_wincodec")))]
    CreateImageSourceFromWic: usize,
    pub CreateLookupTable3D: unsafe extern "system" fn(*mut core::ffi::c_void, super::d2d1_1::D2D1_BUFFER_PRECISION, *const u32, *const u8, u32, *const u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgicommon"))]
    pub CreateImageSourceFromDxgi: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32, super::dxgicommon::DXGI_COLOR_SPACE_TYPE, D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgi", feature = "Win32_dxgicommon")))]
    CreateImageSourceFromDxgi: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub GetGradientMeshWorldBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::dcommon::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    GetGradientMeshWorldBounds: usize,
    pub DrawInk: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
    pub DrawGradientMesh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(feature = "Win32_dcommon")]
    pub DrawGdiMetafile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::dcommon::D2D_RECT_F, *const super::dcommon::D2D_RECT_F),
    #[cfg(not(feature = "Win32_dcommon"))]
    DrawGdiMetafile: usize,
    pub CreateTransformedImageSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec"))]
pub trait ID2D1DeviceContext2_Impl: super::d2d1_2::ID2D1DeviceContext1_Impl {
    fn CreateInk(&self, startpoint: *const D2D1_INK_POINT) -> windows_core::Result<ID2D1Ink>;
    fn CreateInkStyle(&self, inkstyleproperties: *const D2D1_INK_STYLE_PROPERTIES) -> windows_core::Result<ID2D1InkStyle>;
    fn CreateGradientMesh(&self, patches: *const D2D1_GRADIENT_MESH_PATCH, patchescount: u32) -> windows_core::Result<ID2D1GradientMesh>;
    fn CreateImageSourceFromWic(&self, wicbitmapsource: windows_core::Ref<super::wincodec::IWICBitmapSource>, loadingoptions: D2D1_IMAGE_SOURCE_LOADING_OPTIONS, alphamode: super::dcommon::D2D1_ALPHA_MODE) -> windows_core::Result<ID2D1ImageSourceFromWic>;
    fn CreateLookupTable3D(&self, precision: super::d2d1_1::D2D1_BUFFER_PRECISION, extents: *const u32, data: *const u8, datacount: u32, strides: *const u32) -> windows_core::Result<ID2D1LookupTable3D>;
    fn CreateImageSourceFromDxgi(&self, surfaces: *const Option<super::dxgi::IDXGISurface>, surfacecount: u32, colorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE, options: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS) -> windows_core::Result<ID2D1ImageSource>;
    fn GetGradientMeshWorldBounds(&self, gradientmesh: windows_core::Ref<ID2D1GradientMesh>) -> windows_core::Result<super::dcommon::D2D_RECT_F>;
    fn DrawInk(&self, ink: windows_core::Ref<ID2D1Ink>, brush: windows_core::Ref<super::d2d1::ID2D1Brush>, inkstyle: windows_core::Ref<ID2D1InkStyle>);
    fn DrawGradientMesh(&self, gradientmesh: windows_core::Ref<ID2D1GradientMesh>);
    fn DrawGdiMetafile(&self, gdimetafile: windows_core::Ref<super::d2d1_1::ID2D1GdiMetafile>, destinationrectangle: *const super::dcommon::D2D_RECT_F, sourcerectangle: *const super::dcommon::D2D_RECT_F);
    fn CreateTransformedImageSource(&self, imagesource: windows_core::Ref<ID2D1ImageSource>, properties: *const D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES) -> windows_core::Result<ID2D1TransformedImageSource>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec"))]
impl ID2D1DeviceContext2_Vtbl {
    pub const fn new<Identity: ID2D1DeviceContext2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInk<Identity: ID2D1DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startpoint: *const D2D1_INK_POINT, ink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext2_Impl::CreateInk(this, core::mem::transmute_copy(&startpoint)) {
                    Ok(ok__) => {
                        ink.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateInkStyle<Identity: ID2D1DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inkstyleproperties: *const D2D1_INK_STYLE_PROPERTIES, inkstyle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext2_Impl::CreateInkStyle(this, core::mem::transmute_copy(&inkstyleproperties)) {
                    Ok(ok__) => {
                        inkstyle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateGradientMesh<Identity: ID2D1DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, patches: *const D2D1_GRADIENT_MESH_PATCH, patchescount: u32, gradientmesh: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext2_Impl::CreateGradientMesh(this, core::mem::transmute_copy(&patches), core::mem::transmute_copy(&patchescount)) {
                    Ok(ok__) => {
                        gradientmesh.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateImageSourceFromWic<Identity: ID2D1DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wicbitmapsource: *mut core::ffi::c_void, loadingoptions: D2D1_IMAGE_SOURCE_LOADING_OPTIONS, alphamode: super::dcommon::D2D1_ALPHA_MODE, imagesource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext2_Impl::CreateImageSourceFromWic(this, core::mem::transmute_copy(&wicbitmapsource), core::mem::transmute_copy(&loadingoptions), core::mem::transmute_copy(&alphamode)) {
                    Ok(ok__) => {
                        imagesource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateLookupTable3D<Identity: ID2D1DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, precision: super::d2d1_1::D2D1_BUFFER_PRECISION, extents: *const u32, data: *const u8, datacount: u32, strides: *const u32, lookuptable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext2_Impl::CreateLookupTable3D(this, core::mem::transmute_copy(&precision), core::mem::transmute_copy(&extents), core::mem::transmute_copy(&data), core::mem::transmute_copy(&datacount), core::mem::transmute_copy(&strides)) {
                    Ok(ok__) => {
                        lookuptable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateImageSourceFromDxgi<Identity: ID2D1DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, surfaces: *const *mut core::ffi::c_void, surfacecount: u32, colorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE, options: D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS, imagesource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext2_Impl::CreateImageSourceFromDxgi(this, core::mem::transmute_copy(&surfaces), core::mem::transmute_copy(&surfacecount), core::mem::transmute_copy(&colorspace), core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        imagesource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGradientMeshWorldBounds<Identity: ID2D1DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradientmesh: *mut core::ffi::c_void, pbounds: *mut super::dcommon::D2D_RECT_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext2_Impl::GetGradientMeshWorldBounds(this, core::mem::transmute_copy(&gradientmesh)) {
                    Ok(ok__) => {
                        pbounds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DrawInk<Identity: ID2D1DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ink: *mut core::ffi::c_void, brush: *mut core::ffi::c_void, inkstyle: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext2_Impl::DrawInk(this, core::mem::transmute_copy(&ink), core::mem::transmute_copy(&brush), core::mem::transmute_copy(&inkstyle));
            }
        }
        unsafe extern "system" fn DrawGradientMesh<Identity: ID2D1DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gradientmesh: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext2_Impl::DrawGradientMesh(this, core::mem::transmute_copy(&gradientmesh));
            }
        }
        unsafe extern "system" fn DrawGdiMetafile<Identity: ID2D1DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, gdimetafile: *mut core::ffi::c_void, destinationrectangle: *const super::dcommon::D2D_RECT_F, sourcerectangle: *const super::dcommon::D2D_RECT_F) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext2_Impl::DrawGdiMetafile(this, core::mem::transmute_copy(&gdimetafile), core::mem::transmute_copy(&destinationrectangle), core::mem::transmute_copy(&sourcerectangle));
            }
        }
        unsafe extern "system" fn CreateTransformedImageSource<Identity: ID2D1DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagesource: *mut core::ffi::c_void, properties: *const D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES, transformedimagesource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext2_Impl::CreateTransformedImageSource(this, core::mem::transmute_copy(&imagesource), core::mem::transmute_copy(&properties)) {
                    Ok(ok__) => {
                        transformedimagesource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::d2d1_2::ID2D1DeviceContext1_Vtbl::new::<Identity, OFFSET>(),
            CreateInk: CreateInk::<Identity, OFFSET>,
            CreateInkStyle: CreateInkStyle::<Identity, OFFSET>,
            CreateGradientMesh: CreateGradientMesh::<Identity, OFFSET>,
            CreateImageSourceFromWic: CreateImageSourceFromWic::<Identity, OFFSET>,
            CreateLookupTable3D: CreateLookupTable3D::<Identity, OFFSET>,
            CreateImageSourceFromDxgi: CreateImageSourceFromDxgi::<Identity, OFFSET>,
            GetGradientMeshWorldBounds: GetGradientMeshWorldBounds::<Identity, OFFSET>,
            DrawInk: DrawInk::<Identity, OFFSET>,
            DrawGradientMesh: DrawGradientMesh::<Identity, OFFSET>,
            DrawGdiMetafile: DrawGdiMetafile::<Identity, OFFSET>,
            CreateTransformedImageSource: CreateTransformedImageSource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1DeviceContext2 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1RenderTarget as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1DeviceContext as windows_core::Interface>::IID || iid == &<super::d2d1_2::ID2D1DeviceContext1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec"))]
impl windows_core::RuntimeName for ID2D1DeviceContext2 {}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::define_interface!(ID2D1DeviceContext3, ID2D1DeviceContext3_Vtbl, 0x235a7496_8351_414c_bcd4_6672ab2d8e00);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl core::ops::Deref for ID2D1DeviceContext3 {
    type Target = ID2D1DeviceContext2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::interface_hierarchy!(ID2D1DeviceContext3, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1::ID2D1RenderTarget, super::d2d1_1::ID2D1DeviceContext, super::d2d1_2::ID2D1DeviceContext1, ID2D1DeviceContext2);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl ID2D1DeviceContext3 {
    pub unsafe fn CreateSpriteBatch(&self) -> windows_core::Result<ID2D1SpriteBatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSpriteBatch)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DrawSpriteBatch<P0, P3>(&self, spritebatch: P0, startindex: u32, spritecount: u32, bitmap: P3, interpolationmode: super::d2d1::D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS)
    where
        P0: windows_core::Param<ID2D1SpriteBatch>,
        P3: windows_core::Param<super::d2d1::ID2D1Bitmap>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawSpriteBatch)(windows_core::Interface::as_raw(self), spritebatch.param().abi(), startindex, spritecount, bitmap.param().abi(), interpolationmode, spriteoptions);
        }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1DeviceContext3_Vtbl {
    pub base__: ID2D1DeviceContext2_Vtbl,
    pub CreateSpriteBatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DrawSpriteBatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, super::d2d1::D2D1_BITMAP_INTERPOLATION_MODE, D2D1_SPRITE_OPTIONS),
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec"))]
pub trait ID2D1DeviceContext3_Impl: ID2D1DeviceContext2_Impl {
    fn CreateSpriteBatch(&self) -> windows_core::Result<ID2D1SpriteBatch>;
    fn DrawSpriteBatch(&self, spritebatch: windows_core::Ref<ID2D1SpriteBatch>, startindex: u32, spritecount: u32, bitmap: windows_core::Ref<super::d2d1::ID2D1Bitmap>, interpolationmode: super::d2d1::D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS);
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec"))]
impl ID2D1DeviceContext3_Vtbl {
    pub const fn new<Identity: ID2D1DeviceContext3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSpriteBatch<Identity: ID2D1DeviceContext3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, spritebatch: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext3_Impl::CreateSpriteBatch(this) {
                    Ok(ok__) => {
                        spritebatch.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DrawSpriteBatch<Identity: ID2D1DeviceContext3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, spritebatch: *mut core::ffi::c_void, startindex: u32, spritecount: u32, bitmap: *mut core::ffi::c_void, interpolationmode: super::d2d1::D2D1_BITMAP_INTERPOLATION_MODE, spriteoptions: D2D1_SPRITE_OPTIONS) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext3_Impl::DrawSpriteBatch(this, core::mem::transmute_copy(&spritebatch), core::mem::transmute_copy(&startindex), core::mem::transmute_copy(&spritecount), core::mem::transmute_copy(&bitmap), core::mem::transmute_copy(&interpolationmode), core::mem::transmute_copy(&spriteoptions));
            }
        }
        Self {
            base__: ID2D1DeviceContext2_Vtbl::new::<Identity, OFFSET>(),
            CreateSpriteBatch: CreateSpriteBatch::<Identity, OFFSET>,
            DrawSpriteBatch: DrawSpriteBatch::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1DeviceContext3 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1RenderTarget as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1DeviceContext as windows_core::Interface>::IID || iid == &<super::d2d1_2::ID2D1DeviceContext1 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec"))]
impl windows_core::RuntimeName for ID2D1DeviceContext3 {}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::define_interface!(ID2D1DeviceContext4, ID2D1DeviceContext4_Vtbl, 0x8c427831_3d90_4476_b647_c4fae349e4db);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl core::ops::Deref for ID2D1DeviceContext4 {
    type Target = ID2D1DeviceContext3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::interface_hierarchy!(ID2D1DeviceContext4, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1::ID2D1RenderTarget, super::d2d1_1::ID2D1DeviceContext, super::d2d1_2::ID2D1DeviceContext1, ID2D1DeviceContext2, ID2D1DeviceContext3);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl ID2D1DeviceContext4 {
    pub unsafe fn CreateSvgGlyphStyle(&self) -> windows_core::Result<ID2D1SvgGlyphStyle> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSvgGlyphStyle)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite"))]
    pub unsafe fn DrawText<P2, P4, P5>(&self, string: &[u16], textformat: P2, layoutrect: *const super::dcommon::D2D_RECT_F, defaultfillbrush: P4, svgglyphstyle: P5, colorpaletteindex: u32, options: super::d2d1::D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::dcommon::DWRITE_MEASURING_MODE)
    where
        P2: windows_core::Param<super::dwrite::IDWriteTextFormat>,
        P4: windows_core::Param<super::d2d1::ID2D1Brush>,
        P5: windows_core::Param<ID2D1SvgGlyphStyle>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawText)(windows_core::Interface::as_raw(self), core::mem::transmute(string.as_ptr()), string.len().try_into().unwrap(), textformat.param().abi(), layoutrect, defaultfillbrush.param().abi(), svgglyphstyle.param().abi(), colorpaletteindex, options, measuringmode);
        }
    }
    #[cfg(feature = "Win32_dwrite")]
    pub unsafe fn DrawTextLayout<P1, P2, P3>(&self, origin: windows_numerics::Vector2, textlayout: P1, defaultfillbrush: P2, svgglyphstyle: P3, colorpaletteindex: u32, options: super::d2d1::D2D1_DRAW_TEXT_OPTIONS)
    where
        P1: windows_core::Param<super::dwrite::IDWriteTextLayout>,
        P2: windows_core::Param<super::d2d1::ID2D1Brush>,
        P3: windows_core::Param<ID2D1SvgGlyphStyle>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawTextLayout)(windows_core::Interface::as_raw(self), core::mem::transmute(origin), textlayout.param().abi(), defaultfillbrush.param().abi(), svgglyphstyle.param().abi(), colorpaletteindex, options);
        }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite"))]
    pub unsafe fn DrawColorBitmapGlyphRun(&self, glyphimageformat: super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, baselineorigin: windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, bitmapsnapoption: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION) {
        unsafe {
            (windows_core::Interface::vtable(self).DrawColorBitmapGlyphRun)(windows_core::Interface::as_raw(self), glyphimageformat, core::mem::transmute(baselineorigin), core::mem::transmute(glyphrun), measuringmode, bitmapsnapoption);
        }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite"))]
    pub unsafe fn DrawSvgGlyphRun<P2, P3>(&self, baselineorigin: windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, defaultfillbrush: P2, svgglyphstyle: P3, colorpaletteindex: u32, measuringmode: super::dcommon::DWRITE_MEASURING_MODE)
    where
        P2: windows_core::Param<super::d2d1::ID2D1Brush>,
        P3: windows_core::Param<ID2D1SvgGlyphStyle>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawSvgGlyphRun)(windows_core::Interface::as_raw(self), core::mem::transmute(baselineorigin), core::mem::transmute(glyphrun), defaultfillbrush.param().abi(), svgglyphstyle.param().abi(), colorpaletteindex, measuringmode);
        }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite"))]
    pub unsafe fn GetColorBitmapGlyphImage<P2>(&self, glyphimageformat: super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, glyphorigin: windows_numerics::Vector2, fontface: P2, fontemsize: f32, glyphindex: u16, issideways: bool, worldtransform: Option<*const windows_numerics::Matrix3x2>, dpix: f32, dpiy: f32, glyphtransform: *mut windows_numerics::Matrix3x2, glyphimage: *mut Option<super::d2d1::ID2D1Image>) -> windows_core::HRESULT
    where
        P2: windows_core::Param<super::dwrite::IDWriteFontFace>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetColorBitmapGlyphImage)(windows_core::Interface::as_raw(self), glyphimageformat, core::mem::transmute(glyphorigin), fontface.param().abi(), fontemsize, glyphindex, issideways.into(), worldtransform.unwrap_or(core::mem::zeroed()) as _, dpix, dpiy, glyphtransform as _, core::mem::transmute(glyphimage)) }
    }
    #[cfg(feature = "Win32_dwrite")]
    pub unsafe fn GetSvgGlyphImage<P1, P6, P7>(&self, glyphorigin: windows_numerics::Vector2, fontface: P1, fontemsize: f32, glyphindex: u16, issideways: bool, worldtransform: Option<*const windows_numerics::Matrix3x2>, defaultfillbrush: P6, svgglyphstyle: P7, colorpaletteindex: u32, glyphtransform: *mut windows_numerics::Matrix3x2, glyphimage: *mut Option<super::d2d1_1::ID2D1CommandList>) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::dwrite::IDWriteFontFace>,
        P6: windows_core::Param<super::d2d1::ID2D1Brush>,
        P7: windows_core::Param<ID2D1SvgGlyphStyle>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetSvgGlyphImage)(windows_core::Interface::as_raw(self), core::mem::transmute(glyphorigin), fontface.param().abi(), fontemsize, glyphindex, issideways.into(), worldtransform.unwrap_or(core::mem::zeroed()) as _, defaultfillbrush.param().abi(), svgglyphstyle.param().abi(), colorpaletteindex, glyphtransform as _, core::mem::transmute(glyphimage)) }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1DeviceContext4_Vtbl {
    pub base__: ID2D1DeviceContext3_Vtbl,
    pub CreateSvgGlyphStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite"))]
    pub DrawText: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, u32, *mut core::ffi::c_void, *const super::dcommon::D2D_RECT_F, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::d2d1::D2D1_DRAW_TEXT_OPTIONS, super::dcommon::DWRITE_MEASURING_MODE),
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dwrite")))]
    DrawText: usize,
    #[cfg(feature = "Win32_dwrite")]
    pub DrawTextLayout: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::d2d1::D2D1_DRAW_TEXT_OPTIONS),
    #[cfg(not(feature = "Win32_dwrite"))]
    DrawTextLayout: usize,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite"))]
    pub DrawColorBitmapGlyphRun: unsafe extern "system" fn(*mut core::ffi::c_void, super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, windows_numerics::Vector2, *const super::dwrite::DWRITE_GLYPH_RUN, super::dcommon::DWRITE_MEASURING_MODE, D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION),
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dwrite")))]
    DrawColorBitmapGlyphRun: usize,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite"))]
    pub DrawSvgGlyphRun: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2, *const super::dwrite::DWRITE_GLYPH_RUN, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::dcommon::DWRITE_MEASURING_MODE),
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dwrite")))]
    DrawSvgGlyphRun: usize,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite"))]
    pub GetColorBitmapGlyphImage: unsafe extern "system" fn(*mut core::ffi::c_void, super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, windows_numerics::Vector2, *mut core::ffi::c_void, f32, u16, windows_core::BOOL, *const windows_numerics::Matrix3x2, f32, f32, *mut windows_numerics::Matrix3x2, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dwrite")))]
    GetColorBitmapGlyphImage: usize,
    #[cfg(feature = "Win32_dwrite")]
    pub GetSvgGlyphImage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2, *mut core::ffi::c_void, f32, u16, windows_core::BOOL, *const windows_numerics::Matrix3x2, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut windows_numerics::Matrix3x2, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dwrite"))]
    GetSvgGlyphImage: usize,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec"))]
pub trait ID2D1DeviceContext4_Impl: ID2D1DeviceContext3_Impl {
    fn CreateSvgGlyphStyle(&self) -> windows_core::Result<ID2D1SvgGlyphStyle>;
    fn DrawText(&self, string: *const u16, stringlength: u32, textformat: windows_core::Ref<super::dwrite::IDWriteTextFormat>, layoutrect: *const super::dcommon::D2D_RECT_F, defaultfillbrush: windows_core::Ref<super::d2d1::ID2D1Brush>, svgglyphstyle: windows_core::Ref<ID2D1SvgGlyphStyle>, colorpaletteindex: u32, options: super::d2d1::D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::dcommon::DWRITE_MEASURING_MODE);
    fn DrawTextLayout(&self, origin: &windows_numerics::Vector2, textlayout: windows_core::Ref<super::dwrite::IDWriteTextLayout>, defaultfillbrush: windows_core::Ref<super::d2d1::ID2D1Brush>, svgglyphstyle: windows_core::Ref<ID2D1SvgGlyphStyle>, colorpaletteindex: u32, options: super::d2d1::D2D1_DRAW_TEXT_OPTIONS);
    fn DrawColorBitmapGlyphRun(&self, glyphimageformat: super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, baselineorigin: &windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, bitmapsnapoption: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION);
    fn DrawSvgGlyphRun(&self, baselineorigin: &windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, defaultfillbrush: windows_core::Ref<super::d2d1::ID2D1Brush>, svgglyphstyle: windows_core::Ref<ID2D1SvgGlyphStyle>, colorpaletteindex: u32, measuringmode: super::dcommon::DWRITE_MEASURING_MODE);
    fn GetColorBitmapGlyphImage(&self, glyphimageformat: super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, glyphorigin: &windows_numerics::Vector2, fontface: windows_core::Ref<super::dwrite::IDWriteFontFace>, fontemsize: f32, glyphindex: u16, issideways: windows_core::BOOL, worldtransform: *const windows_numerics::Matrix3x2, dpix: f32, dpiy: f32, glyphtransform: *mut windows_numerics::Matrix3x2, glyphimage: windows_core::OutRef<super::d2d1::ID2D1Image>) -> windows_core::Result<()>;
    fn GetSvgGlyphImage(&self, glyphorigin: &windows_numerics::Vector2, fontface: windows_core::Ref<super::dwrite::IDWriteFontFace>, fontemsize: f32, glyphindex: u16, issideways: windows_core::BOOL, worldtransform: *const windows_numerics::Matrix3x2, defaultfillbrush: windows_core::Ref<super::d2d1::ID2D1Brush>, svgglyphstyle: windows_core::Ref<ID2D1SvgGlyphStyle>, colorpaletteindex: u32, glyphtransform: *mut windows_numerics::Matrix3x2, glyphimage: windows_core::OutRef<super::d2d1_1::ID2D1CommandList>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec"))]
impl ID2D1DeviceContext4_Vtbl {
    pub const fn new<Identity: ID2D1DeviceContext4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSvgGlyphStyle<Identity: ID2D1DeviceContext4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, svgglyphstyle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext4_Impl::CreateSvgGlyphStyle(this) {
                    Ok(ok__) => {
                        svgglyphstyle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DrawText<Identity: ID2D1DeviceContext4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, string: *const u16, stringlength: u32, textformat: *mut core::ffi::c_void, layoutrect: *const super::dcommon::D2D_RECT_F, defaultfillbrush: *mut core::ffi::c_void, svgglyphstyle: *mut core::ffi::c_void, colorpaletteindex: u32, options: super::d2d1::D2D1_DRAW_TEXT_OPTIONS, measuringmode: super::dcommon::DWRITE_MEASURING_MODE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext4_Impl::DrawText(this, core::mem::transmute_copy(&string), core::mem::transmute_copy(&stringlength), core::mem::transmute_copy(&textformat), core::mem::transmute_copy(&layoutrect), core::mem::transmute_copy(&defaultfillbrush), core::mem::transmute_copy(&svgglyphstyle), core::mem::transmute_copy(&colorpaletteindex), core::mem::transmute_copy(&options), core::mem::transmute_copy(&measuringmode));
            }
        }
        unsafe extern "system" fn DrawTextLayout<Identity: ID2D1DeviceContext4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, origin: windows_numerics::Vector2, textlayout: *mut core::ffi::c_void, defaultfillbrush: *mut core::ffi::c_void, svgglyphstyle: *mut core::ffi::c_void, colorpaletteindex: u32, options: super::d2d1::D2D1_DRAW_TEXT_OPTIONS) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext4_Impl::DrawTextLayout(this, core::mem::transmute(&origin), core::mem::transmute_copy(&textlayout), core::mem::transmute_copy(&defaultfillbrush), core::mem::transmute_copy(&svgglyphstyle), core::mem::transmute_copy(&colorpaletteindex), core::mem::transmute_copy(&options));
            }
        }
        unsafe extern "system" fn DrawColorBitmapGlyphRun<Identity: ID2D1DeviceContext4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphimageformat: super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, baselineorigin: windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, bitmapsnapoption: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext4_Impl::DrawColorBitmapGlyphRun(this, core::mem::transmute_copy(&glyphimageformat), core::mem::transmute(&baselineorigin), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&bitmapsnapoption));
            }
        }
        unsafe extern "system" fn DrawSvgGlyphRun<Identity: ID2D1DeviceContext4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineorigin: windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, defaultfillbrush: *mut core::ffi::c_void, svgglyphstyle: *mut core::ffi::c_void, colorpaletteindex: u32, measuringmode: super::dcommon::DWRITE_MEASURING_MODE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext4_Impl::DrawSvgGlyphRun(this, core::mem::transmute(&baselineorigin), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&defaultfillbrush), core::mem::transmute_copy(&svgglyphstyle), core::mem::transmute_copy(&colorpaletteindex), core::mem::transmute_copy(&measuringmode));
            }
        }
        unsafe extern "system" fn GetColorBitmapGlyphImage<Identity: ID2D1DeviceContext4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphimageformat: super::dcommon::DWRITE_GLYPH_IMAGE_FORMATS, glyphorigin: windows_numerics::Vector2, fontface: *mut core::ffi::c_void, fontemsize: f32, glyphindex: u16, issideways: windows_core::BOOL, worldtransform: *const windows_numerics::Matrix3x2, dpix: f32, dpiy: f32, glyphtransform: *mut windows_numerics::Matrix3x2, glyphimage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext4_Impl::GetColorBitmapGlyphImage(this, core::mem::transmute_copy(&glyphimageformat), core::mem::transmute(&glyphorigin), core::mem::transmute_copy(&fontface), core::mem::transmute_copy(&fontemsize), core::mem::transmute_copy(&glyphindex), core::mem::transmute_copy(&issideways), core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy), core::mem::transmute_copy(&glyphtransform), core::mem::transmute_copy(&glyphimage)).into()
            }
        }
        unsafe extern "system" fn GetSvgGlyphImage<Identity: ID2D1DeviceContext4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, glyphorigin: windows_numerics::Vector2, fontface: *mut core::ffi::c_void, fontemsize: f32, glyphindex: u16, issideways: windows_core::BOOL, worldtransform: *const windows_numerics::Matrix3x2, defaultfillbrush: *mut core::ffi::c_void, svgglyphstyle: *mut core::ffi::c_void, colorpaletteindex: u32, glyphtransform: *mut windows_numerics::Matrix3x2, glyphimage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext4_Impl::GetSvgGlyphImage(this, core::mem::transmute(&glyphorigin), core::mem::transmute_copy(&fontface), core::mem::transmute_copy(&fontemsize), core::mem::transmute_copy(&glyphindex), core::mem::transmute_copy(&issideways), core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&defaultfillbrush), core::mem::transmute_copy(&svgglyphstyle), core::mem::transmute_copy(&colorpaletteindex), core::mem::transmute_copy(&glyphtransform), core::mem::transmute_copy(&glyphimage)).into()
            }
        }
        Self {
            base__: ID2D1DeviceContext3_Vtbl::new::<Identity, OFFSET>(),
            CreateSvgGlyphStyle: CreateSvgGlyphStyle::<Identity, OFFSET>,
            DrawText: DrawText::<Identity, OFFSET>,
            DrawTextLayout: DrawTextLayout::<Identity, OFFSET>,
            DrawColorBitmapGlyphRun: DrawColorBitmapGlyphRun::<Identity, OFFSET>,
            DrawSvgGlyphRun: DrawSvgGlyphRun::<Identity, OFFSET>,
            GetColorBitmapGlyphImage: GetColorBitmapGlyphImage::<Identity, OFFSET>,
            GetSvgGlyphImage: GetSvgGlyphImage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1DeviceContext4 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1RenderTarget as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1DeviceContext as windows_core::Interface>::IID || iid == &<super::d2d1_2::ID2D1DeviceContext1 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext2 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec"))]
impl windows_core::RuntimeName for ID2D1DeviceContext4 {}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::define_interface!(ID2D1DeviceContext5, ID2D1DeviceContext5_Vtbl, 0x7836d248_68cc_4df6_b9e8_de991bf62eb7);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl core::ops::Deref for ID2D1DeviceContext5 {
    type Target = ID2D1DeviceContext4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::interface_hierarchy!(ID2D1DeviceContext5, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1::ID2D1RenderTarget, super::d2d1_1::ID2D1DeviceContext, super::d2d1_2::ID2D1DeviceContext1, ID2D1DeviceContext2, ID2D1DeviceContext3, ID2D1DeviceContext4);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl ID2D1DeviceContext5 {
    #[cfg(all(feature = "Win32_d2d1svg", feature = "Win32_dcommon", feature = "Win32_objidlbase"))]
    pub unsafe fn CreateSvgDocument<P0>(&self, inputxmlstream: P0, viewportsize: super::dcommon::D2D_SIZE_F) -> windows_core::Result<super::d2d1svg::ID2D1SvgDocument>
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSvgDocument)(windows_core::Interface::as_raw(self), inputxmlstream.param().abi(), core::mem::transmute(viewportsize), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_d2d1svg")]
    pub unsafe fn DrawSvgDocument<P0>(&self, svgdocument: P0)
    where
        P0: windows_core::Param<super::d2d1svg::ID2D1SvgDocument>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawSvgDocument)(windows_core::Interface::as_raw(self), svgdocument.param().abi());
        }
    }
    #[cfg(feature = "Win32_dxgicommon")]
    pub unsafe fn CreateColorContextFromDxgiColorSpace(&self, colorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<ID2D1ColorContext1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateColorContextFromDxgiColorSpace)(windows_core::Interface::as_raw(self), colorspace, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateColorContextFromSimpleColorProfile(&self, simpleprofile: *const D2D1_SIMPLE_COLOR_PROFILE) -> windows_core::Result<ID2D1ColorContext1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateColorContextFromSimpleColorProfile)(windows_core::Interface::as_raw(self), simpleprofile, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1DeviceContext5_Vtbl {
    pub base__: ID2D1DeviceContext4_Vtbl,
    #[cfg(all(feature = "Win32_d2d1svg", feature = "Win32_dcommon", feature = "Win32_objidlbase"))]
    pub CreateSvgDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::dcommon::D2D_SIZE_F, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d2d1svg", feature = "Win32_dcommon", feature = "Win32_objidlbase")))]
    CreateSvgDocument: usize,
    #[cfg(feature = "Win32_d2d1svg")]
    pub DrawSvgDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_d2d1svg"))]
    DrawSvgDocument: usize,
    #[cfg(feature = "Win32_dxgicommon")]
    pub CreateColorContextFromDxgiColorSpace: unsafe extern "system" fn(*mut core::ffi::c_void, super::dxgicommon::DXGI_COLOR_SPACE_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgicommon"))]
    CreateColorContextFromDxgiColorSpace: usize,
    pub CreateColorContextFromSimpleColorProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_SIMPLE_COLOR_PROFILE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2d1svg", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_objidlbase", feature = "Win32_wincodec"))]
pub trait ID2D1DeviceContext5_Impl: ID2D1DeviceContext4_Impl {
    fn CreateSvgDocument(&self, inputxmlstream: windows_core::Ref<super::objidlbase::IStream>, viewportsize: &super::dcommon::D2D_SIZE_F) -> windows_core::Result<super::d2d1svg::ID2D1SvgDocument>;
    fn DrawSvgDocument(&self, svgdocument: windows_core::Ref<super::d2d1svg::ID2D1SvgDocument>);
    fn CreateColorContextFromDxgiColorSpace(&self, colorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<ID2D1ColorContext1>;
    fn CreateColorContextFromSimpleColorProfile(&self, simpleprofile: *const D2D1_SIMPLE_COLOR_PROFILE) -> windows_core::Result<ID2D1ColorContext1>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2d1svg", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_objidlbase", feature = "Win32_wincodec"))]
impl ID2D1DeviceContext5_Vtbl {
    pub const fn new<Identity: ID2D1DeviceContext5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSvgDocument<Identity: ID2D1DeviceContext5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputxmlstream: *mut core::ffi::c_void, viewportsize: super::dcommon::D2D_SIZE_F, svgdocument: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext5_Impl::CreateSvgDocument(this, core::mem::transmute_copy(&inputxmlstream), core::mem::transmute(&viewportsize)) {
                    Ok(ok__) => {
                        svgdocument.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DrawSvgDocument<Identity: ID2D1DeviceContext5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, svgdocument: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext5_Impl::DrawSvgDocument(this, core::mem::transmute_copy(&svgdocument));
            }
        }
        unsafe extern "system" fn CreateColorContextFromDxgiColorSpace<Identity: ID2D1DeviceContext5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE, colorcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext5_Impl::CreateColorContextFromDxgiColorSpace(this, core::mem::transmute_copy(&colorspace)) {
                    Ok(ok__) => {
                        colorcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateColorContextFromSimpleColorProfile<Identity: ID2D1DeviceContext5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, simpleprofile: *const D2D1_SIMPLE_COLOR_PROFILE, colorcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext5_Impl::CreateColorContextFromSimpleColorProfile(this, core::mem::transmute_copy(&simpleprofile)) {
                    Ok(ok__) => {
                        colorcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ID2D1DeviceContext4_Vtbl::new::<Identity, OFFSET>(),
            CreateSvgDocument: CreateSvgDocument::<Identity, OFFSET>,
            DrawSvgDocument: DrawSvgDocument::<Identity, OFFSET>,
            CreateColorContextFromDxgiColorSpace: CreateColorContextFromDxgiColorSpace::<Identity, OFFSET>,
            CreateColorContextFromSimpleColorProfile: CreateColorContextFromSimpleColorProfile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1DeviceContext5 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1RenderTarget as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1DeviceContext as windows_core::Interface>::IID || iid == &<super::d2d1_2::ID2D1DeviceContext1 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext2 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext3 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2d1svg", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_objidlbase", feature = "Win32_wincodec"))]
impl windows_core::RuntimeName for ID2D1DeviceContext5 {}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::define_interface!(ID2D1DeviceContext6, ID2D1DeviceContext6_Vtbl, 0x985f7e37_4ed0_4a19_98a3_15b0edfde306);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl core::ops::Deref for ID2D1DeviceContext6 {
    type Target = ID2D1DeviceContext5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::interface_hierarchy!(ID2D1DeviceContext6, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1::ID2D1RenderTarget, super::d2d1_1::ID2D1DeviceContext, super::d2d1_2::ID2D1DeviceContext1, ID2D1DeviceContext2, ID2D1DeviceContext3, ID2D1DeviceContext4, ID2D1DeviceContext5);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl ID2D1DeviceContext6 {
    #[cfg(all(feature = "Win32_d2d1effects", feature = "Win32_dcommon"))]
    pub unsafe fn BlendImage<P0>(&self, image: P0, blendmode: super::d2d1effects::D2D1_BLEND_MODE, targetoffset: Option<*const windows_numerics::Vector2>, imagerectangle: Option<*const super::dcommon::D2D_RECT_F>, interpolationmode: super::d2d1_1::D2D1_INTERPOLATION_MODE)
    where
        P0: windows_core::Param<super::d2d1::ID2D1Image>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).BlendImage)(windows_core::Interface::as_raw(self), image.param().abi(), blendmode, targetoffset.unwrap_or(core::mem::zeroed()) as _, imagerectangle.unwrap_or(core::mem::zeroed()) as _, interpolationmode);
        }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1DeviceContext6_Vtbl {
    pub base__: ID2D1DeviceContext5_Vtbl,
    #[cfg(all(feature = "Win32_d2d1effects", feature = "Win32_dcommon"))]
    pub BlendImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::d2d1effects::D2D1_BLEND_MODE, *const windows_numerics::Vector2, *const super::dcommon::D2D_RECT_F, super::d2d1_1::D2D1_INTERPOLATION_MODE),
    #[cfg(not(all(feature = "Win32_d2d1effects", feature = "Win32_dcommon")))]
    BlendImage: usize,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2d1effects", feature = "Win32_d2d1svg", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_objidlbase", feature = "Win32_wincodec"))]
pub trait ID2D1DeviceContext6_Impl: ID2D1DeviceContext5_Impl {
    fn BlendImage(&self, image: windows_core::Ref<super::d2d1::ID2D1Image>, blendmode: super::d2d1effects::D2D1_BLEND_MODE, targetoffset: *const windows_numerics::Vector2, imagerectangle: *const super::dcommon::D2D_RECT_F, interpolationmode: super::d2d1_1::D2D1_INTERPOLATION_MODE);
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2d1effects", feature = "Win32_d2d1svg", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_objidlbase", feature = "Win32_wincodec"))]
impl ID2D1DeviceContext6_Vtbl {
    pub const fn new<Identity: ID2D1DeviceContext6_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BlendImage<Identity: ID2D1DeviceContext6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, image: *mut core::ffi::c_void, blendmode: super::d2d1effects::D2D1_BLEND_MODE, targetoffset: *const windows_numerics::Vector2, imagerectangle: *const super::dcommon::D2D_RECT_F, interpolationmode: super::d2d1_1::D2D1_INTERPOLATION_MODE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext6_Impl::BlendImage(this, core::mem::transmute_copy(&image), core::mem::transmute_copy(&blendmode), core::mem::transmute_copy(&targetoffset), core::mem::transmute_copy(&imagerectangle), core::mem::transmute_copy(&interpolationmode));
            }
        }
        Self { base__: ID2D1DeviceContext5_Vtbl::new::<Identity, OFFSET>(), BlendImage: BlendImage::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1DeviceContext6 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1RenderTarget as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1DeviceContext as windows_core::Interface>::IID || iid == &<super::d2d1_2::ID2D1DeviceContext1 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext2 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext3 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext4 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext5 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2d1effects", feature = "Win32_d2d1svg", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_objidlbase", feature = "Win32_wincodec"))]
impl windows_core::RuntimeName for ID2D1DeviceContext6 {}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::define_interface!(ID2D1DeviceContext7, ID2D1DeviceContext7_Vtbl, 0xec891cf7_9b69_4851_9def_4e0915771e62);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl core::ops::Deref for ID2D1DeviceContext7 {
    type Target = ID2D1DeviceContext6;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::interface_hierarchy!(ID2D1DeviceContext7, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1::ID2D1RenderTarget, super::d2d1_1::ID2D1DeviceContext, super::d2d1_2::ID2D1DeviceContext1, ID2D1DeviceContext2, ID2D1DeviceContext3, ID2D1DeviceContext4, ID2D1DeviceContext5, ID2D1DeviceContext6);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl ID2D1DeviceContext7 {
    #[cfg(feature = "Win32_dwrite_3")]
    pub unsafe fn GetPaintFeatureLevel(&self) -> super::dwrite_3::DWRITE_PAINT_FEATURE_LEVEL {
        unsafe { (windows_core::Interface::vtable(self).GetPaintFeatureLevel)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite"))]
    pub unsafe fn DrawPaintGlyphRun<P2>(&self, baselineorigin: windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, defaultfillbrush: P2, colorpaletteindex: u32, measuringmode: super::dcommon::DWRITE_MEASURING_MODE)
    where
        P2: windows_core::Param<super::d2d1::ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawPaintGlyphRun)(windows_core::Interface::as_raw(self), core::mem::transmute(baselineorigin), core::mem::transmute(glyphrun), defaultfillbrush.param().abi(), colorpaletteindex, measuringmode);
        }
    }
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite"))]
    pub unsafe fn DrawGlyphRunWithColorSupport<P3, P4>(&self, baselineorigin: windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, glyphrundescription: Option<*const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION>, foregroundbrush: P3, svgglyphstyle: P4, colorpaletteindex: u32, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, bitmapsnapoption: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION)
    where
        P3: windows_core::Param<super::d2d1::ID2D1Brush>,
        P4: windows_core::Param<ID2D1SvgGlyphStyle>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawGlyphRunWithColorSupport)(windows_core::Interface::as_raw(self), core::mem::transmute(baselineorigin), core::mem::transmute(glyphrun), glyphrundescription.unwrap_or(core::mem::zeroed()) as _, foregroundbrush.param().abi(), svgglyphstyle.param().abi(), colorpaletteindex, measuringmode, bitmapsnapoption);
        }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1DeviceContext7_Vtbl {
    pub base__: ID2D1DeviceContext6_Vtbl,
    #[cfg(feature = "Win32_dwrite_3")]
    pub GetPaintFeatureLevel: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::dwrite_3::DWRITE_PAINT_FEATURE_LEVEL,
    #[cfg(not(feature = "Win32_dwrite_3"))]
    GetPaintFeatureLevel: usize,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite"))]
    pub DrawPaintGlyphRun: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2, *const super::dwrite::DWRITE_GLYPH_RUN, *mut core::ffi::c_void, u32, super::dcommon::DWRITE_MEASURING_MODE),
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dwrite")))]
    DrawPaintGlyphRun: usize,
    #[cfg(all(feature = "Win32_dcommon", feature = "Win32_dwrite"))]
    pub DrawGlyphRunWithColorSupport: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2, *const super::dwrite::DWRITE_GLYPH_RUN, *const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::dcommon::DWRITE_MEASURING_MODE, D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION),
    #[cfg(not(all(feature = "Win32_dcommon", feature = "Win32_dwrite")))]
    DrawGlyphRunWithColorSupport: usize,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2d1effects", feature = "Win32_d2d1svg", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_3", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_objidlbase", feature = "Win32_wincodec"))]
pub trait ID2D1DeviceContext7_Impl: ID2D1DeviceContext6_Impl {
    fn GetPaintFeatureLevel(&self) -> super::dwrite_3::DWRITE_PAINT_FEATURE_LEVEL;
    fn DrawPaintGlyphRun(&self, baselineorigin: &windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, defaultfillbrush: windows_core::Ref<super::d2d1::ID2D1Brush>, colorpaletteindex: u32, measuringmode: super::dcommon::DWRITE_MEASURING_MODE);
    fn DrawGlyphRunWithColorSupport(&self, baselineorigin: &windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION, foregroundbrush: windows_core::Ref<super::d2d1::ID2D1Brush>, svgglyphstyle: windows_core::Ref<ID2D1SvgGlyphStyle>, colorpaletteindex: u32, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, bitmapsnapoption: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION);
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2d1effects", feature = "Win32_d2d1svg", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_3", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_objidlbase", feature = "Win32_wincodec"))]
impl ID2D1DeviceContext7_Vtbl {
    pub const fn new<Identity: ID2D1DeviceContext7_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPaintFeatureLevel<Identity: ID2D1DeviceContext7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::dwrite_3::DWRITE_PAINT_FEATURE_LEVEL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext7_Impl::GetPaintFeatureLevel(this)
            }
        }
        unsafe extern "system" fn DrawPaintGlyphRun<Identity: ID2D1DeviceContext7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineorigin: windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, defaultfillbrush: *mut core::ffi::c_void, colorpaletteindex: u32, measuringmode: super::dcommon::DWRITE_MEASURING_MODE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext7_Impl::DrawPaintGlyphRun(this, core::mem::transmute(&baselineorigin), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&defaultfillbrush), core::mem::transmute_copy(&colorpaletteindex), core::mem::transmute_copy(&measuringmode));
            }
        }
        unsafe extern "system" fn DrawGlyphRunWithColorSupport<Identity: ID2D1DeviceContext7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baselineorigin: windows_numerics::Vector2, glyphrun: *const super::dwrite::DWRITE_GLYPH_RUN, glyphrundescription: *const super::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION, foregroundbrush: *mut core::ffi::c_void, svgglyphstyle: *mut core::ffi::c_void, colorpaletteindex: u32, measuringmode: super::dcommon::DWRITE_MEASURING_MODE, bitmapsnapoption: D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext7_Impl::DrawGlyphRunWithColorSupport(this, core::mem::transmute(&baselineorigin), core::mem::transmute_copy(&glyphrun), core::mem::transmute_copy(&glyphrundescription), core::mem::transmute_copy(&foregroundbrush), core::mem::transmute_copy(&svgglyphstyle), core::mem::transmute_copy(&colorpaletteindex), core::mem::transmute_copy(&measuringmode), core::mem::transmute_copy(&bitmapsnapoption));
            }
        }
        Self {
            base__: ID2D1DeviceContext6_Vtbl::new::<Identity, OFFSET>(),
            GetPaintFeatureLevel: GetPaintFeatureLevel::<Identity, OFFSET>,
            DrawPaintGlyphRun: DrawPaintGlyphRun::<Identity, OFFSET>,
            DrawGlyphRunWithColorSupport: DrawGlyphRunWithColorSupport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1DeviceContext7 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1RenderTarget as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1DeviceContext as windows_core::Interface>::IID || iid == &<super::d2d1_2::ID2D1DeviceContext1 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext2 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext3 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext4 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext5 as windows_core::Interface>::IID || iid == &<ID2D1DeviceContext6 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_d2d1effects", feature = "Win32_d2d1svg", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dwrite_3", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_objidlbase", feature = "Win32_wincodec"))]
impl windows_core::RuntimeName for ID2D1DeviceContext7 {}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::define_interface!(ID2D1Factory3, ID2D1Factory3_Vtbl, 0x0869759f_4f00_413f_b03e_2bda45404d0f);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl core::ops::Deref for ID2D1Factory3 {
    type Target = super::d2d1_2::ID2D1Factory2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::interface_hierarchy!(ID2D1Factory3, windows_core::IUnknown, super::d2d1::ID2D1Factory, super::d2d1_1::ID2D1Factory1, super::d2d1_2::ID2D1Factory2);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl ID2D1Factory3 {
    #[cfg(feature = "Win32_dxgi")]
    pub unsafe fn CreateDevice<P0>(&self, dxgidevice: P0) -> windows_core::Result<ID2D1Device2>
    where
        P0: windows_core::Param<super::dxgi::IDXGIDevice>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), dxgidevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Factory3_Vtbl {
    pub base__: super::d2d1_2::ID2D1Factory2_Vtbl,
    #[cfg(feature = "Win32_dxgi")]
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgi"))]
    CreateDevice: usize,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_objidlbase", feature = "Win32_wincodec", feature = "Win32_windef"))]
pub trait ID2D1Factory3_Impl: super::d2d1_2::ID2D1Factory2_Impl {
    fn CreateDevice(&self, dxgidevice: windows_core::Ref<super::dxgi::IDXGIDevice>) -> windows_core::Result<ID2D1Device2>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_objidlbase", feature = "Win32_wincodec", feature = "Win32_windef"))]
impl ID2D1Factory3_Vtbl {
    pub const fn new<Identity: ID2D1Factory3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDevice<Identity: ID2D1Factory3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxgidevice: *mut core::ffi::c_void, d2ddevice2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory3_Impl::CreateDevice(this, core::mem::transmute_copy(&dxgidevice)) {
                    Ok(ok__) => {
                        d2ddevice2.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::d2d1_2::ID2D1Factory2_Vtbl::new::<Identity, OFFSET>(), CreateDevice: CreateDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Factory3 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Factory as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1Factory1 as windows_core::Interface>::IID || iid == &<super::d2d1_2::ID2D1Factory2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_objidlbase", feature = "Win32_wincodec", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID2D1Factory3 {}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::define_interface!(ID2D1Factory4, ID2D1Factory4_Vtbl, 0xbd4ec2d2_0662_4bee_ba8e_6f29f032e096);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl core::ops::Deref for ID2D1Factory4 {
    type Target = ID2D1Factory3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::interface_hierarchy!(ID2D1Factory4, windows_core::IUnknown, super::d2d1::ID2D1Factory, super::d2d1_1::ID2D1Factory1, super::d2d1_2::ID2D1Factory2, ID2D1Factory3);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl ID2D1Factory4 {
    #[cfg(feature = "Win32_dxgi")]
    pub unsafe fn CreateDevice<P0>(&self, dxgidevice: P0) -> windows_core::Result<ID2D1Device3>
    where
        P0: windows_core::Param<super::dxgi::IDXGIDevice>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), dxgidevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Factory4_Vtbl {
    pub base__: ID2D1Factory3_Vtbl,
    #[cfg(feature = "Win32_dxgi")]
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgi"))]
    CreateDevice: usize,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_objidlbase", feature = "Win32_wincodec", feature = "Win32_windef"))]
pub trait ID2D1Factory4_Impl: ID2D1Factory3_Impl {
    fn CreateDevice(&self, dxgidevice: windows_core::Ref<super::dxgi::IDXGIDevice>) -> windows_core::Result<ID2D1Device3>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_objidlbase", feature = "Win32_wincodec", feature = "Win32_windef"))]
impl ID2D1Factory4_Vtbl {
    pub const fn new<Identity: ID2D1Factory4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDevice<Identity: ID2D1Factory4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxgidevice: *mut core::ffi::c_void, d2ddevice3: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory4_Impl::CreateDevice(this, core::mem::transmute_copy(&dxgidevice)) {
                    Ok(ok__) => {
                        d2ddevice3.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ID2D1Factory3_Vtbl::new::<Identity, OFFSET>(), CreateDevice: CreateDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Factory4 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Factory as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1Factory1 as windows_core::Interface>::IID || iid == &<super::d2d1_2::ID2D1Factory2 as windows_core::Interface>::IID || iid == &<ID2D1Factory3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_objidlbase", feature = "Win32_wincodec", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID2D1Factory4 {}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::define_interface!(ID2D1Factory5, ID2D1Factory5_Vtbl, 0xc4349994_838e_4b0f_8cab_44997d9eeacc);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl core::ops::Deref for ID2D1Factory5 {
    type Target = ID2D1Factory4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::interface_hierarchy!(ID2D1Factory5, windows_core::IUnknown, super::d2d1::ID2D1Factory, super::d2d1_1::ID2D1Factory1, super::d2d1_2::ID2D1Factory2, ID2D1Factory3, ID2D1Factory4);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl ID2D1Factory5 {
    #[cfg(feature = "Win32_dxgi")]
    pub unsafe fn CreateDevice<P0>(&self, dxgidevice: P0) -> windows_core::Result<ID2D1Device4>
    where
        P0: windows_core::Param<super::dxgi::IDXGIDevice>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), dxgidevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Factory5_Vtbl {
    pub base__: ID2D1Factory4_Vtbl,
    #[cfg(feature = "Win32_dxgi")]
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgi"))]
    CreateDevice: usize,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_objidlbase", feature = "Win32_wincodec", feature = "Win32_windef"))]
pub trait ID2D1Factory5_Impl: ID2D1Factory4_Impl {
    fn CreateDevice(&self, dxgidevice: windows_core::Ref<super::dxgi::IDXGIDevice>) -> windows_core::Result<ID2D1Device4>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_objidlbase", feature = "Win32_wincodec", feature = "Win32_windef"))]
impl ID2D1Factory5_Vtbl {
    pub const fn new<Identity: ID2D1Factory5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDevice<Identity: ID2D1Factory5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxgidevice: *mut core::ffi::c_void, d2ddevice4: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory5_Impl::CreateDevice(this, core::mem::transmute_copy(&dxgidevice)) {
                    Ok(ok__) => {
                        d2ddevice4.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ID2D1Factory4_Vtbl::new::<Identity, OFFSET>(), CreateDevice: CreateDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Factory5 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Factory as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1Factory1 as windows_core::Interface>::IID || iid == &<super::d2d1_2::ID2D1Factory2 as windows_core::Interface>::IID || iid == &<ID2D1Factory3 as windows_core::Interface>::IID || iid == &<ID2D1Factory4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_objidlbase", feature = "Win32_wincodec", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID2D1Factory5 {}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::define_interface!(ID2D1Factory6, ID2D1Factory6_Vtbl, 0xf9976f46_f642_44c1_97ca_da32ea2a2635);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl core::ops::Deref for ID2D1Factory6 {
    type Target = ID2D1Factory5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::interface_hierarchy!(ID2D1Factory6, windows_core::IUnknown, super::d2d1::ID2D1Factory, super::d2d1_1::ID2D1Factory1, super::d2d1_2::ID2D1Factory2, ID2D1Factory3, ID2D1Factory4, ID2D1Factory5);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl ID2D1Factory6 {
    #[cfg(feature = "Win32_dxgi")]
    pub unsafe fn CreateDevice<P0>(&self, dxgidevice: P0) -> windows_core::Result<ID2D1Device5>
    where
        P0: windows_core::Param<super::dxgi::IDXGIDevice>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), dxgidevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Factory6_Vtbl {
    pub base__: ID2D1Factory5_Vtbl,
    #[cfg(feature = "Win32_dxgi")]
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgi"))]
    CreateDevice: usize,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_objidlbase", feature = "Win32_wincodec", feature = "Win32_windef"))]
pub trait ID2D1Factory6_Impl: ID2D1Factory5_Impl {
    fn CreateDevice(&self, dxgidevice: windows_core::Ref<super::dxgi::IDXGIDevice>) -> windows_core::Result<ID2D1Device5>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_objidlbase", feature = "Win32_wincodec", feature = "Win32_windef"))]
impl ID2D1Factory6_Vtbl {
    pub const fn new<Identity: ID2D1Factory6_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDevice<Identity: ID2D1Factory6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxgidevice: *mut core::ffi::c_void, d2ddevice5: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory6_Impl::CreateDevice(this, core::mem::transmute_copy(&dxgidevice)) {
                    Ok(ok__) => {
                        d2ddevice5.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ID2D1Factory5_Vtbl::new::<Identity, OFFSET>(), CreateDevice: CreateDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Factory6 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Factory as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1Factory1 as windows_core::Interface>::IID || iid == &<super::d2d1_2::ID2D1Factory2 as windows_core::Interface>::IID || iid == &<ID2D1Factory3 as windows_core::Interface>::IID || iid == &<ID2D1Factory4 as windows_core::Interface>::IID || iid == &<ID2D1Factory5 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_objidlbase", feature = "Win32_wincodec", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID2D1Factory6 {}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::define_interface!(ID2D1Factory7, ID2D1Factory7_Vtbl, 0xbdc2bdd3_b96c_4de6_bdf7_99d4745454de);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl core::ops::Deref for ID2D1Factory7 {
    type Target = ID2D1Factory6;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::interface_hierarchy!(ID2D1Factory7, windows_core::IUnknown, super::d2d1::ID2D1Factory, super::d2d1_1::ID2D1Factory1, super::d2d1_2::ID2D1Factory2, ID2D1Factory3, ID2D1Factory4, ID2D1Factory5, ID2D1Factory6);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl ID2D1Factory7 {
    #[cfg(feature = "Win32_dxgi")]
    pub unsafe fn CreateDevice<P0>(&self, dxgidevice: P0) -> windows_core::Result<ID2D1Device6>
    where
        P0: windows_core::Param<super::dxgi::IDXGIDevice>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), dxgidevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Factory7_Vtbl {
    pub base__: ID2D1Factory6_Vtbl,
    #[cfg(feature = "Win32_dxgi")]
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgi"))]
    CreateDevice: usize,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_objidlbase", feature = "Win32_wincodec", feature = "Win32_windef"))]
pub trait ID2D1Factory7_Impl: ID2D1Factory6_Impl {
    fn CreateDevice(&self, dxgidevice: windows_core::Ref<super::dxgi::IDXGIDevice>) -> windows_core::Result<ID2D1Device6>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_objidlbase", feature = "Win32_wincodec", feature = "Win32_windef"))]
impl ID2D1Factory7_Vtbl {
    pub const fn new<Identity: ID2D1Factory7_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDevice<Identity: ID2D1Factory7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxgidevice: *mut core::ffi::c_void, d2ddevice6: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory7_Impl::CreateDevice(this, core::mem::transmute_copy(&dxgidevice)) {
                    Ok(ok__) => {
                        d2ddevice6.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ID2D1Factory6_Vtbl::new::<Identity, OFFSET>(), CreateDevice: CreateDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Factory7 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Factory as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1Factory1 as windows_core::Interface>::IID || iid == &<super::d2d1_2::ID2D1Factory2 as windows_core::Interface>::IID || iid == &<ID2D1Factory3 as windows_core::Interface>::IID || iid == &<ID2D1Factory4 as windows_core::Interface>::IID || iid == &<ID2D1Factory5 as windows_core::Interface>::IID || iid == &<ID2D1Factory6 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_objidlbase", feature = "Win32_wincodec", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID2D1Factory7 {}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::define_interface!(ID2D1Factory8, ID2D1Factory8_Vtbl, 0x677c9311_f36d_4b1f_ae86_86d1223ffd3a);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl core::ops::Deref for ID2D1Factory8 {
    type Target = ID2D1Factory7;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
windows_core::imp::interface_hierarchy!(ID2D1Factory8, windows_core::IUnknown, super::d2d1::ID2D1Factory, super::d2d1_1::ID2D1Factory1, super::d2d1_2::ID2D1Factory2, ID2D1Factory3, ID2D1Factory4, ID2D1Factory5, ID2D1Factory6, ID2D1Factory7);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
impl ID2D1Factory8 {
    #[cfg(feature = "Win32_dxgi")]
    pub unsafe fn CreateDevice<P0>(&self, dxgidevice: P0) -> windows_core::Result<ID2D1Device7>
    where
        P0: windows_core::Param<super::dxgi::IDXGIDevice>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), dxgidevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Factory8_Vtbl {
    pub base__: ID2D1Factory7_Vtbl,
    #[cfg(feature = "Win32_dxgi")]
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgi"))]
    CreateDevice: usize,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_objidlbase", feature = "Win32_wincodec", feature = "Win32_windef"))]
pub trait ID2D1Factory8_Impl: ID2D1Factory7_Impl {
    fn CreateDevice(&self, dxgidevice: windows_core::Ref<super::dxgi::IDXGIDevice>) -> windows_core::Result<ID2D1Device7>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_objidlbase", feature = "Win32_wincodec", feature = "Win32_windef"))]
impl ID2D1Factory8_Vtbl {
    pub const fn new<Identity: ID2D1Factory8_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDevice<Identity: ID2D1Factory8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxgidevice: *mut core::ffi::c_void, d2ddevice6: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory8_Impl::CreateDevice(this, core::mem::transmute_copy(&dxgidevice)) {
                    Ok(ok__) => {
                        d2ddevice6.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ID2D1Factory7_Vtbl::new::<Identity, OFFSET>(), CreateDevice: CreateDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Factory8 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Factory as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1Factory1 as windows_core::Interface>::IID || iid == &<super::d2d1_2::ID2D1Factory2 as windows_core::Interface>::IID || iid == &<ID2D1Factory3 as windows_core::Interface>::IID || iid == &<ID2D1Factory4 as windows_core::Interface>::IID || iid == &<ID2D1Factory5 as windows_core::Interface>::IID || iid == &<ID2D1Factory6 as windows_core::Interface>::IID || iid == &<ID2D1Factory7 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2d1_2", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_objidlbase", feature = "Win32_wincodec", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID2D1Factory8 {}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
windows_core::imp::define_interface!(ID2D1GdiMetafile1, ID2D1GdiMetafile1_Vtbl, 0x2e69f9e8_dd3f_4bf9_95ba_c04f49d788df);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
impl core::ops::Deref for ID2D1GdiMetafile1 {
    type Target = super::d2d1_1::ID2D1GdiMetafile;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
windows_core::imp::interface_hierarchy!(ID2D1GdiMetafile1, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1_1::ID2D1GdiMetafile);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
impl ID2D1GdiMetafile1 {
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDpi)(windows_core::Interface::as_raw(self), dpix as _, dpiy as _) }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetSourceBounds(&self) -> windows_core::Result<super::dcommon::D2D_RECT_F> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSourceBounds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1GdiMetafile1_Vtbl {
    pub base__: super::d2d1_1::ID2D1GdiMetafile_Vtbl,
    pub GetDpi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_dcommon")]
    pub GetSourceBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::dcommon::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    GetSourceBounds: usize,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_dcommon"))]
pub trait ID2D1GdiMetafile1_Impl: super::d2d1_1::ID2D1GdiMetafile_Impl {
    fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) -> windows_core::Result<()>;
    fn GetSourceBounds(&self) -> windows_core::Result<super::dcommon::D2D_RECT_F>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_dcommon"))]
impl ID2D1GdiMetafile1_Vtbl {
    pub const fn new<Identity: ID2D1GdiMetafile1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDpi<Identity: ID2D1GdiMetafile1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dpix: *mut f32, dpiy: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GdiMetafile1_Impl::GetDpi(this, core::mem::transmute_copy(&dpix), core::mem::transmute_copy(&dpiy)).into()
            }
        }
        unsafe extern "system" fn GetSourceBounds<Identity: ID2D1GdiMetafile1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bounds: *mut super::dcommon::D2D_RECT_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1GdiMetafile1_Impl::GetSourceBounds(this) {
                    Ok(ok__) => {
                        bounds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::d2d1_1::ID2D1GdiMetafile_Vtbl::new::<Identity, OFFSET>(),
            GetDpi: GetDpi::<Identity, OFFSET>,
            GetSourceBounds: GetSourceBounds::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1GdiMetafile1 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1GdiMetafile as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_dcommon"))]
impl windows_core::RuntimeName for ID2D1GdiMetafile1 {}
#[cfg(feature = "Win32_d2d1_1")]
windows_core::imp::define_interface!(ID2D1GdiMetafileSink1, ID2D1GdiMetafileSink1_Vtbl, 0xfd0ecb6b_91e6_411e_8655_395e760f91b4);
#[cfg(feature = "Win32_d2d1_1")]
impl core::ops::Deref for ID2D1GdiMetafileSink1 {
    type Target = super::d2d1_1::ID2D1GdiMetafileSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1_1")]
windows_core::imp::interface_hierarchy!(ID2D1GdiMetafileSink1, windows_core::IUnknown, super::d2d1_1::ID2D1GdiMetafileSink);
#[cfg(feature = "Win32_d2d1_1")]
impl ID2D1GdiMetafileSink1 {
    pub unsafe fn ProcessRecord(&self, recordtype: u32, recorddata: Option<*const core::ffi::c_void>, recorddatasize: u32, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessRecord)(windows_core::Interface::as_raw(self), recordtype, recorddata.unwrap_or(core::mem::zeroed()) as _, recorddatasize, flags) }
    }
}
#[cfg(feature = "Win32_d2d1_1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1GdiMetafileSink1_Vtbl {
    pub base__: super::d2d1_1::ID2D1GdiMetafileSink_Vtbl,
    pub ProcessRecord: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_d2d1_1")]
pub trait ID2D1GdiMetafileSink1_Impl: super::d2d1_1::ID2D1GdiMetafileSink_Impl {
    fn ProcessRecord(&self, recordtype: u32, recorddata: *const core::ffi::c_void, recorddatasize: u32, flags: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_d2d1_1")]
impl ID2D1GdiMetafileSink1_Vtbl {
    pub const fn new<Identity: ID2D1GdiMetafileSink1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProcessRecord<Identity: ID2D1GdiMetafileSink1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, recordtype: u32, recorddata: *const core::ffi::c_void, recorddatasize: u32, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GdiMetafileSink1_Impl::ProcessRecord(this, core::mem::transmute_copy(&recordtype), core::mem::transmute_copy(&recorddata), core::mem::transmute_copy(&recorddatasize), core::mem::transmute_copy(&flags)).into()
            }
        }
        Self { base__: super::d2d1_1::ID2D1GdiMetafileSink_Vtbl::new::<Identity, OFFSET>(), ProcessRecord: ProcessRecord::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1GdiMetafileSink1 as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1GdiMetafileSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d2d1_1")]
impl windows_core::RuntimeName for ID2D1GdiMetafileSink1 {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1GradientMesh, ID2D1GradientMesh_Vtbl, 0xf292e401_c050_4cde_83d7_04962d3b23c2);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1GradientMesh {
    type Target = super::d2d1::ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1GradientMesh, windows_core::IUnknown, super::d2d1::ID2D1Resource);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1GradientMesh {
    pub unsafe fn GetPatchCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetPatchCount)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub unsafe fn GetPatches(&self, startindex: u32, patches: &mut [D2D1_GRADIENT_MESH_PATCH]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPatches)(windows_core::Interface::as_raw(self), startindex, core::mem::transmute(patches.as_ptr()), patches.len().try_into().unwrap()) }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1GradientMesh_Vtbl {
    pub base__: super::d2d1::ID2D1Resource_Vtbl,
    pub GetPatchCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub GetPatches: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D2D1_GRADIENT_MESH_PATCH, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype")))]
    GetPatches: usize,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
pub trait ID2D1GradientMesh_Impl: super::d2d1::ID2D1Resource_Impl {
    fn GetPatchCount(&self) -> u32;
    fn GetPatches(&self, startindex: u32, patches: *mut D2D1_GRADIENT_MESH_PATCH, patchescount: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
impl ID2D1GradientMesh_Vtbl {
    pub const fn new<Identity: ID2D1GradientMesh_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPatchCount<Identity: ID2D1GradientMesh_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GradientMesh_Impl::GetPatchCount(this)
            }
        }
        unsafe extern "system" fn GetPatches<Identity: ID2D1GradientMesh_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startindex: u32, patches: *mut D2D1_GRADIENT_MESH_PATCH, patchescount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GradientMesh_Impl::GetPatches(this, core::mem::transmute_copy(&startindex), core::mem::transmute_copy(&patches), core::mem::transmute_copy(&patchescount)).into()
            }
        }
        Self {
            base__: super::d2d1::ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            GetPatchCount: GetPatchCount::<Identity, OFFSET>,
            GetPatches: GetPatches::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1GradientMesh as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
impl windows_core::RuntimeName for ID2D1GradientMesh {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1ImageSource, ID2D1ImageSource_Vtbl, 0xc9b664e5_74a1_4378_9ac2_eefc37a3f4d8);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1ImageSource {
    type Target = super::d2d1::ID2D1Image;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1ImageSource, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1::ID2D1Image);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1ImageSource {
    pub unsafe fn OfferResources(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OfferResources)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn TryReclaimResources(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryReclaimResources)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1ImageSource_Vtbl {
    pub base__: super::d2d1::ID2D1Image_Vtbl,
    pub OfferResources: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryReclaimResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_d2d1")]
pub trait ID2D1ImageSource_Impl: super::d2d1::ID2D1Image_Impl {
    fn OfferResources(&self) -> windows_core::Result<()>;
    fn TryReclaimResources(&self) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(feature = "Win32_d2d1")]
impl ID2D1ImageSource_Vtbl {
    pub const fn new<Identity: ID2D1ImageSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OfferResources<Identity: ID2D1ImageSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1ImageSource_Impl::OfferResources(this).into()
            }
        }
        unsafe extern "system" fn TryReclaimResources<Identity: ID2D1ImageSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcesdiscarded: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1ImageSource_Impl::TryReclaimResources(this) {
                    Ok(ok__) => {
                        resourcesdiscarded.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::d2d1::ID2D1Image_Vtbl::new::<Identity, OFFSET>(),
            OfferResources: OfferResources::<Identity, OFFSET>,
            TryReclaimResources: TryReclaimResources::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1ImageSource as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Image as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d2d1")]
impl windows_core::RuntimeName for ID2D1ImageSource {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1ImageSourceFromWic, ID2D1ImageSourceFromWic_Vtbl, 0x77395441_1c8f_4555_8683_f50dab0fe792);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1ImageSourceFromWic {
    type Target = ID2D1ImageSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1ImageSourceFromWic, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1::ID2D1Image, ID2D1ImageSource);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1ImageSourceFromWic {
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn EnsureCached(&self, rectangletofill: Option<*const super::dcommon::D2D_RECT_U>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnsureCached)(windows_core::Interface::as_raw(self), rectangletofill.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn TrimCache(&self, rectangletopreserve: Option<*const super::dcommon::D2D_RECT_U>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TrimCache)(windows_core::Interface::as_raw(self), rectangletopreserve.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_wincodec")]
    pub unsafe fn GetSource(&self) -> windows_core::Result<super::wincodec::IWICBitmapSource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSource)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1ImageSourceFromWic_Vtbl {
    pub base__: ID2D1ImageSource_Vtbl,
    #[cfg(feature = "Win32_dcommon")]
    pub EnsureCached: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dcommon::D2D_RECT_U) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    EnsureCached: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub TrimCache: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::dcommon::D2D_RECT_U) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    TrimCache: usize,
    #[cfg(feature = "Win32_wincodec")]
    pub GetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_wincodec"))]
    GetSource: usize,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_wincodec"))]
pub trait ID2D1ImageSourceFromWic_Impl: ID2D1ImageSource_Impl {
    fn EnsureCached(&self, rectangletofill: *const super::dcommon::D2D_RECT_U) -> windows_core::Result<()>;
    fn TrimCache(&self, rectangletopreserve: *const super::dcommon::D2D_RECT_U) -> windows_core::Result<()>;
    fn GetSource(&self, wicbitmapsource: windows_core::OutRef<super::wincodec::IWICBitmapSource>);
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_wincodec"))]
impl ID2D1ImageSourceFromWic_Vtbl {
    pub const fn new<Identity: ID2D1ImageSourceFromWic_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnsureCached<Identity: ID2D1ImageSourceFromWic_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rectangletofill: *const super::dcommon::D2D_RECT_U) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1ImageSourceFromWic_Impl::EnsureCached(this, core::mem::transmute_copy(&rectangletofill)).into()
            }
        }
        unsafe extern "system" fn TrimCache<Identity: ID2D1ImageSourceFromWic_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rectangletopreserve: *const super::dcommon::D2D_RECT_U) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1ImageSourceFromWic_Impl::TrimCache(this, core::mem::transmute_copy(&rectangletopreserve)).into()
            }
        }
        unsafe extern "system" fn GetSource<Identity: ID2D1ImageSourceFromWic_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wicbitmapsource: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1ImageSourceFromWic_Impl::GetSource(this, core::mem::transmute_copy(&wicbitmapsource));
            }
        }
        Self {
            base__: ID2D1ImageSource_Vtbl::new::<Identity, OFFSET>(),
            EnsureCached: EnsureCached::<Identity, OFFSET>,
            TrimCache: TrimCache::<Identity, OFFSET>,
            GetSource: GetSource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1ImageSourceFromWic as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Image as windows_core::Interface>::IID || iid == &<ID2D1ImageSource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon", feature = "Win32_wincodec"))]
impl windows_core::RuntimeName for ID2D1ImageSourceFromWic {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1Ink, ID2D1Ink_Vtbl, 0xb499923b_7029_478f_a8b3_432c7c5f5312);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1Ink {
    type Target = super::d2d1::ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1Ink, windows_core::IUnknown, super::d2d1::ID2D1Resource);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1Ink {
    pub unsafe fn SetStartPoint(&self, startpoint: *const D2D1_INK_POINT) {
        unsafe {
            (windows_core::Interface::vtable(self).SetStartPoint)(windows_core::Interface::as_raw(self), startpoint);
        }
    }
    pub unsafe fn GetStartPoint(&self) -> D2D1_INK_POINT {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStartPoint)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn AddSegments(&self, segments: &[D2D1_INK_BEZIER_SEGMENT]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddSegments)(windows_core::Interface::as_raw(self), core::mem::transmute(segments.as_ptr()), segments.len().try_into().unwrap()) }
    }
    pub unsafe fn RemoveSegmentsAtEnd(&self, segmentscount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveSegmentsAtEnd)(windows_core::Interface::as_raw(self), segmentscount) }
    }
    pub unsafe fn SetSegments(&self, startsegment: u32, segments: &[D2D1_INK_BEZIER_SEGMENT]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSegments)(windows_core::Interface::as_raw(self), startsegment, core::mem::transmute(segments.as_ptr()), segments.len().try_into().unwrap()) }
    }
    pub unsafe fn SetSegmentAtEnd(&self, segment: *const D2D1_INK_BEZIER_SEGMENT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSegmentAtEnd)(windows_core::Interface::as_raw(self), segment) }
    }
    pub unsafe fn GetSegmentCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetSegmentCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetSegments(&self, startsegment: u32, segments: &mut [D2D1_INK_BEZIER_SEGMENT]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSegments)(windows_core::Interface::as_raw(self), startsegment, core::mem::transmute(segments.as_ptr()), segments.len().try_into().unwrap()) }
    }
    pub unsafe fn StreamAsGeometry<P0, P3>(&self, inkstyle: P0, worldtransform: Option<*const windows_numerics::Matrix3x2>, flatteningtolerance: f32, geometrysink: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID2D1InkStyle>,
        P3: windows_core::Param<super::d2d1::ID2D1SimplifiedGeometrySink>,
    {
        unsafe { (windows_core::Interface::vtable(self).StreamAsGeometry)(windows_core::Interface::as_raw(self), inkstyle.param().abi(), worldtransform.unwrap_or(core::mem::zeroed()) as _, flatteningtolerance, geometrysink.param().abi()) }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetBounds<P0>(&self, inkstyle: P0, worldtransform: Option<*const windows_numerics::Matrix3x2>) -> windows_core::Result<super::dcommon::D2D_RECT_F>
    where
        P0: windows_core::Param<ID2D1InkStyle>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBounds)(windows_core::Interface::as_raw(self), inkstyle.param().abi(), worldtransform.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Ink_Vtbl {
    pub base__: super::d2d1::ID2D1Resource_Vtbl,
    pub SetStartPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_INK_POINT),
    pub GetStartPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_INK_POINT),
    pub AddSegments: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_INK_BEZIER_SEGMENT, u32) -> windows_core::HRESULT,
    pub RemoveSegmentsAtEnd: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetSegments: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D2D1_INK_BEZIER_SEGMENT, u32) -> windows_core::HRESULT,
    pub SetSegmentAtEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_INK_BEZIER_SEGMENT) -> windows_core::HRESULT,
    pub GetSegmentCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetSegments: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut D2D1_INK_BEZIER_SEGMENT, u32) -> windows_core::HRESULT,
    pub StreamAsGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_numerics::Matrix3x2, f32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_dcommon")]
    pub GetBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_numerics::Matrix3x2, *mut super::dcommon::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    GetBounds: usize,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
pub trait ID2D1Ink_Impl: super::d2d1::ID2D1Resource_Impl {
    fn SetStartPoint(&self, startpoint: *const D2D1_INK_POINT);
    fn GetStartPoint(&self) -> D2D1_INK_POINT;
    fn AddSegments(&self, segments: *const D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> windows_core::Result<()>;
    fn RemoveSegmentsAtEnd(&self, segmentscount: u32) -> windows_core::Result<()>;
    fn SetSegments(&self, startsegment: u32, segments: *const D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> windows_core::Result<()>;
    fn SetSegmentAtEnd(&self, segment: *const D2D1_INK_BEZIER_SEGMENT) -> windows_core::Result<()>;
    fn GetSegmentCount(&self) -> u32;
    fn GetSegments(&self, startsegment: u32, segments: *mut D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> windows_core::Result<()>;
    fn StreamAsGeometry(&self, inkstyle: windows_core::Ref<ID2D1InkStyle>, worldtransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: windows_core::Ref<super::d2d1::ID2D1SimplifiedGeometrySink>) -> windows_core::Result<()>;
    fn GetBounds(&self, inkstyle: windows_core::Ref<ID2D1InkStyle>, worldtransform: *const windows_numerics::Matrix3x2) -> windows_core::Result<super::dcommon::D2D_RECT_F>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
impl ID2D1Ink_Vtbl {
    pub const fn new<Identity: ID2D1Ink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetStartPoint<Identity: ID2D1Ink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startpoint: *const D2D1_INK_POINT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Ink_Impl::SetStartPoint(this, core::mem::transmute_copy(&startpoint));
            }
        }
        unsafe extern "system" fn GetStartPoint<Identity: ID2D1Ink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut D2D1_INK_POINT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                *result__ = ID2D1Ink_Impl::GetStartPoint(this);
            }
        }
        unsafe extern "system" fn AddSegments<Identity: ID2D1Ink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, segments: *const D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Ink_Impl::AddSegments(this, core::mem::transmute_copy(&segments), core::mem::transmute_copy(&segmentscount)).into()
            }
        }
        unsafe extern "system" fn RemoveSegmentsAtEnd<Identity: ID2D1Ink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentscount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Ink_Impl::RemoveSegmentsAtEnd(this, core::mem::transmute_copy(&segmentscount)).into()
            }
        }
        unsafe extern "system" fn SetSegments<Identity: ID2D1Ink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startsegment: u32, segments: *const D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Ink_Impl::SetSegments(this, core::mem::transmute_copy(&startsegment), core::mem::transmute_copy(&segments), core::mem::transmute_copy(&segmentscount)).into()
            }
        }
        unsafe extern "system" fn SetSegmentAtEnd<Identity: ID2D1Ink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, segment: *const D2D1_INK_BEZIER_SEGMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Ink_Impl::SetSegmentAtEnd(this, core::mem::transmute_copy(&segment)).into()
            }
        }
        unsafe extern "system" fn GetSegmentCount<Identity: ID2D1Ink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Ink_Impl::GetSegmentCount(this)
            }
        }
        unsafe extern "system" fn GetSegments<Identity: ID2D1Ink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startsegment: u32, segments: *mut D2D1_INK_BEZIER_SEGMENT, segmentscount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Ink_Impl::GetSegments(this, core::mem::transmute_copy(&startsegment), core::mem::transmute_copy(&segments), core::mem::transmute_copy(&segmentscount)).into()
            }
        }
        unsafe extern "system" fn StreamAsGeometry<Identity: ID2D1Ink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inkstyle: *mut core::ffi::c_void, worldtransform: *const windows_numerics::Matrix3x2, flatteningtolerance: f32, geometrysink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Ink_Impl::StreamAsGeometry(this, core::mem::transmute_copy(&inkstyle), core::mem::transmute_copy(&worldtransform), core::mem::transmute_copy(&flatteningtolerance), core::mem::transmute_copy(&geometrysink)).into()
            }
        }
        unsafe extern "system" fn GetBounds<Identity: ID2D1Ink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inkstyle: *mut core::ffi::c_void, worldtransform: *const windows_numerics::Matrix3x2, bounds: *mut super::dcommon::D2D_RECT_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Ink_Impl::GetBounds(this, core::mem::transmute_copy(&inkstyle), core::mem::transmute_copy(&worldtransform)) {
                    Ok(ok__) => {
                        bounds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::d2d1::ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            SetStartPoint: SetStartPoint::<Identity, OFFSET>,
            GetStartPoint: GetStartPoint::<Identity, OFFSET>,
            AddSegments: AddSegments::<Identity, OFFSET>,
            RemoveSegmentsAtEnd: RemoveSegmentsAtEnd::<Identity, OFFSET>,
            SetSegments: SetSegments::<Identity, OFFSET>,
            SetSegmentAtEnd: SetSegmentAtEnd::<Identity, OFFSET>,
            GetSegmentCount: GetSegmentCount::<Identity, OFFSET>,
            GetSegments: GetSegments::<Identity, OFFSET>,
            StreamAsGeometry: StreamAsGeometry::<Identity, OFFSET>,
            GetBounds: GetBounds::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Ink as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_dcommon"))]
impl windows_core::RuntimeName for ID2D1Ink {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1InkStyle, ID2D1InkStyle_Vtbl, 0xbae8b344_23fc_4071_8cb5_d05d6f073848);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1InkStyle {
    type Target = super::d2d1::ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1InkStyle, windows_core::IUnknown, super::d2d1::ID2D1Resource);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1InkStyle {
    pub unsafe fn SetNibTransform(&self, transform: *const windows_numerics::Matrix3x2) {
        unsafe {
            (windows_core::Interface::vtable(self).SetNibTransform)(windows_core::Interface::as_raw(self), transform);
        }
    }
    pub unsafe fn GetNibTransform(&self, transform: *mut windows_numerics::Matrix3x2) {
        unsafe {
            (windows_core::Interface::vtable(self).GetNibTransform)(windows_core::Interface::as_raw(self), transform as _);
        }
    }
    pub unsafe fn SetNibShape(&self, nibshape: D2D1_INK_NIB_SHAPE) {
        unsafe {
            (windows_core::Interface::vtable(self).SetNibShape)(windows_core::Interface::as_raw(self), nibshape);
        }
    }
    pub unsafe fn GetNibShape(&self) -> D2D1_INK_NIB_SHAPE {
        unsafe { (windows_core::Interface::vtable(self).GetNibShape)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1InkStyle_Vtbl {
    pub base__: super::d2d1::ID2D1Resource_Vtbl,
    pub SetNibTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Matrix3x2),
    pub GetNibTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Matrix3x2),
    pub SetNibShape: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_INK_NIB_SHAPE),
    pub GetNibShape: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_INK_NIB_SHAPE,
}
#[cfg(feature = "Win32_d2d1")]
pub trait ID2D1InkStyle_Impl: super::d2d1::ID2D1Resource_Impl {
    fn SetNibTransform(&self, transform: *const windows_numerics::Matrix3x2);
    fn GetNibTransform(&self, transform: *mut windows_numerics::Matrix3x2);
    fn SetNibShape(&self, nibshape: D2D1_INK_NIB_SHAPE);
    fn GetNibShape(&self) -> D2D1_INK_NIB_SHAPE;
}
#[cfg(feature = "Win32_d2d1")]
impl ID2D1InkStyle_Vtbl {
    pub const fn new<Identity: ID2D1InkStyle_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetNibTransform<Identity: ID2D1InkStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *const windows_numerics::Matrix3x2) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1InkStyle_Impl::SetNibTransform(this, core::mem::transmute_copy(&transform));
            }
        }
        unsafe extern "system" fn GetNibTransform<Identity: ID2D1InkStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut windows_numerics::Matrix3x2) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1InkStyle_Impl::GetNibTransform(this, core::mem::transmute_copy(&transform));
            }
        }
        unsafe extern "system" fn SetNibShape<Identity: ID2D1InkStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nibshape: D2D1_INK_NIB_SHAPE) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1InkStyle_Impl::SetNibShape(this, core::mem::transmute_copy(&nibshape));
            }
        }
        unsafe extern "system" fn GetNibShape<Identity: ID2D1InkStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_INK_NIB_SHAPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1InkStyle_Impl::GetNibShape(this)
            }
        }
        Self {
            base__: super::d2d1::ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            SetNibTransform: SetNibTransform::<Identity, OFFSET>,
            GetNibTransform: GetNibTransform::<Identity, OFFSET>,
            SetNibShape: SetNibShape::<Identity, OFFSET>,
            GetNibShape: GetNibShape::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1InkStyle as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d2d1")]
impl windows_core::RuntimeName for ID2D1InkStyle {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1LookupTable3D, ID2D1LookupTable3D_Vtbl, 0x53dd9855_a3b0_4d5b_82e1_26e25c5e5797);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1LookupTable3D {
    type Target = super::d2d1::ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1LookupTable3D, windows_core::IUnknown, super::d2d1::ID2D1Resource);
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1LookupTable3D_Vtbl {
    pub base__: super::d2d1::ID2D1Resource_Vtbl,
}
#[cfg(feature = "Win32_d2d1")]
pub trait ID2D1LookupTable3D_Impl: super::d2d1::ID2D1Resource_Impl {}
#[cfg(feature = "Win32_d2d1")]
impl ID2D1LookupTable3D_Vtbl {
    pub const fn new<Identity: ID2D1LookupTable3D_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::d2d1::ID2D1Resource_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1LookupTable3D as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d2d1")]
impl windows_core::RuntimeName for ID2D1LookupTable3D {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1SpriteBatch, ID2D1SpriteBatch_Vtbl, 0x4dc583bf_3a10_438a_8722_e9765224f1f1);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1SpriteBatch {
    type Target = super::d2d1::ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1SpriteBatch, windows_core::IUnknown, super::d2d1::ID2D1Resource);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1SpriteBatch {
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dxgitype"))]
    pub unsafe fn AddSprites(&self, spritecount: u32, destinationrectangles: *const super::dcommon::D2D_RECT_F, sourcerectangles: Option<*const super::dcommon::D2D_RECT_U>, colors: Option<*const super::d2dbasetypes::D2D_COLOR_F>, transforms: Option<*const windows_numerics::Matrix3x2>, destinationrectanglesstride: u32, sourcerectanglesstride: u32, colorsstride: u32, transformsstride: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddSprites)(windows_core::Interface::as_raw(self), spritecount, destinationrectangles, sourcerectangles.unwrap_or(core::mem::zeroed()) as _, colors.unwrap_or(core::mem::zeroed()) as _, transforms.unwrap_or(core::mem::zeroed()) as _, destinationrectanglesstride, sourcerectanglesstride, colorsstride, transformsstride) }
    }
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dxgitype"))]
    pub unsafe fn SetSprites(&self, startindex: u32, spritecount: u32, destinationrectangles: Option<*const super::dcommon::D2D_RECT_F>, sourcerectangles: Option<*const super::dcommon::D2D_RECT_U>, colors: Option<*const super::d2dbasetypes::D2D_COLOR_F>, transforms: Option<*const windows_numerics::Matrix3x2>, destinationrectanglesstride: u32, sourcerectanglesstride: u32, colorsstride: u32, transformsstride: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSprites)(windows_core::Interface::as_raw(self), startindex, spritecount, destinationrectangles.unwrap_or(core::mem::zeroed()) as _, sourcerectangles.unwrap_or(core::mem::zeroed()) as _, colors.unwrap_or(core::mem::zeroed()) as _, transforms.unwrap_or(core::mem::zeroed()) as _, destinationrectanglesstride, sourcerectanglesstride, colorsstride, transformsstride) }
    }
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dxgitype"))]
    pub unsafe fn GetSprites(&self, startindex: u32, spritecount: u32, destinationrectangles: Option<*mut super::dcommon::D2D_RECT_F>, sourcerectangles: Option<*mut super::dcommon::D2D_RECT_U>, colors: Option<*mut super::d2dbasetypes::D2D_COLOR_F>, transforms: Option<*mut windows_numerics::Matrix3x2>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSprites)(windows_core::Interface::as_raw(self), startindex, spritecount, destinationrectangles.unwrap_or(core::mem::zeroed()) as _, sourcerectangles.unwrap_or(core::mem::zeroed()) as _, colors.unwrap_or(core::mem::zeroed()) as _, transforms.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetSpriteCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetSpriteCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clear(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self));
        }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1SpriteBatch_Vtbl {
    pub base__: super::d2d1::ID2D1Resource_Vtbl,
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dxgitype"))]
    pub AddSprites: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::dcommon::D2D_RECT_F, *const super::dcommon::D2D_RECT_U, *const super::d2dbasetypes::D2D_COLOR_F, *const windows_numerics::Matrix3x2, u32, u32, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dxgitype")))]
    AddSprites: usize,
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dxgitype"))]
    pub SetSprites: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const super::dcommon::D2D_RECT_F, *const super::dcommon::D2D_RECT_U, *const super::d2dbasetypes::D2D_COLOR_F, *const windows_numerics::Matrix3x2, u32, u32, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dxgitype")))]
    SetSprites: usize,
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dxgitype"))]
    pub GetSprites: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut super::dcommon::D2D_RECT_F, *mut super::dcommon::D2D_RECT_U, *mut super::d2dbasetypes::D2D_COLOR_F, *mut windows_numerics::Matrix3x2) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dxgitype")))]
    GetSprites: usize,
    pub GetSpriteCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void),
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dxgitype"))]
pub trait ID2D1SpriteBatch_Impl: super::d2d1::ID2D1Resource_Impl {
    fn AddSprites(&self, spritecount: u32, destinationrectangles: *const super::dcommon::D2D_RECT_F, sourcerectangles: *const super::dcommon::D2D_RECT_U, colors: *const super::d2dbasetypes::D2D_COLOR_F, transforms: *const windows_numerics::Matrix3x2, destinationrectanglesstride: u32, sourcerectanglesstride: u32, colorsstride: u32, transformsstride: u32) -> windows_core::Result<()>;
    fn SetSprites(&self, startindex: u32, spritecount: u32, destinationrectangles: *const super::dcommon::D2D_RECT_F, sourcerectangles: *const super::dcommon::D2D_RECT_U, colors: *const super::d2dbasetypes::D2D_COLOR_F, transforms: *const windows_numerics::Matrix3x2, destinationrectanglesstride: u32, sourcerectanglesstride: u32, colorsstride: u32, transformsstride: u32) -> windows_core::Result<()>;
    fn GetSprites(&self, startindex: u32, spritecount: u32, destinationrectangles: *mut super::dcommon::D2D_RECT_F, sourcerectangles: *mut super::dcommon::D2D_RECT_U, colors: *mut super::d2dbasetypes::D2D_COLOR_F, transforms: *mut windows_numerics::Matrix3x2) -> windows_core::Result<()>;
    fn GetSpriteCount(&self) -> u32;
    fn Clear(&self);
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dxgitype"))]
impl ID2D1SpriteBatch_Vtbl {
    pub const fn new<Identity: ID2D1SpriteBatch_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddSprites<Identity: ID2D1SpriteBatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, spritecount: u32, destinationrectangles: *const super::dcommon::D2D_RECT_F, sourcerectangles: *const super::dcommon::D2D_RECT_U, colors: *const super::d2dbasetypes::D2D_COLOR_F, transforms: *const windows_numerics::Matrix3x2, destinationrectanglesstride: u32, sourcerectanglesstride: u32, colorsstride: u32, transformsstride: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SpriteBatch_Impl::AddSprites(this, core::mem::transmute_copy(&spritecount), core::mem::transmute_copy(&destinationrectangles), core::mem::transmute_copy(&sourcerectangles), core::mem::transmute_copy(&colors), core::mem::transmute_copy(&transforms), core::mem::transmute_copy(&destinationrectanglesstride), core::mem::transmute_copy(&sourcerectanglesstride), core::mem::transmute_copy(&colorsstride), core::mem::transmute_copy(&transformsstride)).into()
            }
        }
        unsafe extern "system" fn SetSprites<Identity: ID2D1SpriteBatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startindex: u32, spritecount: u32, destinationrectangles: *const super::dcommon::D2D_RECT_F, sourcerectangles: *const super::dcommon::D2D_RECT_U, colors: *const super::d2dbasetypes::D2D_COLOR_F, transforms: *const windows_numerics::Matrix3x2, destinationrectanglesstride: u32, sourcerectanglesstride: u32, colorsstride: u32, transformsstride: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SpriteBatch_Impl::SetSprites(this, core::mem::transmute_copy(&startindex), core::mem::transmute_copy(&spritecount), core::mem::transmute_copy(&destinationrectangles), core::mem::transmute_copy(&sourcerectangles), core::mem::transmute_copy(&colors), core::mem::transmute_copy(&transforms), core::mem::transmute_copy(&destinationrectanglesstride), core::mem::transmute_copy(&sourcerectanglesstride), core::mem::transmute_copy(&colorsstride), core::mem::transmute_copy(&transformsstride)).into()
            }
        }
        unsafe extern "system" fn GetSprites<Identity: ID2D1SpriteBatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startindex: u32, spritecount: u32, destinationrectangles: *mut super::dcommon::D2D_RECT_F, sourcerectangles: *mut super::dcommon::D2D_RECT_U, colors: *mut super::d2dbasetypes::D2D_COLOR_F, transforms: *mut windows_numerics::Matrix3x2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SpriteBatch_Impl::GetSprites(this, core::mem::transmute_copy(&startindex), core::mem::transmute_copy(&spritecount), core::mem::transmute_copy(&destinationrectangles), core::mem::transmute_copy(&sourcerectangles), core::mem::transmute_copy(&colors), core::mem::transmute_copy(&transforms)).into()
            }
        }
        unsafe extern "system" fn GetSpriteCount<Identity: ID2D1SpriteBatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SpriteBatch_Impl::GetSpriteCount(this)
            }
        }
        unsafe extern "system" fn Clear<Identity: ID2D1SpriteBatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SpriteBatch_Impl::Clear(this);
            }
        }
        Self {
            base__: super::d2d1::ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            AddSprites: AddSprites::<Identity, OFFSET>,
            SetSprites: SetSprites::<Identity, OFFSET>,
            GetSprites: GetSprites::<Identity, OFFSET>,
            GetSpriteCount: GetSpriteCount::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1SpriteBatch as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dxgitype"))]
impl windows_core::RuntimeName for ID2D1SpriteBatch {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1SvgGlyphStyle, ID2D1SvgGlyphStyle_Vtbl, 0xaf671749_d241_4db8_8e41_dcc2e5c1a438);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1SvgGlyphStyle {
    type Target = super::d2d1::ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1SvgGlyphStyle, windows_core::IUnknown, super::d2d1::ID2D1Resource);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1SvgGlyphStyle {
    pub unsafe fn SetFill<P0>(&self, brush: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d2d1::ID2D1Brush>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFill)(windows_core::Interface::as_raw(self), brush.param().abi()) }
    }
    pub unsafe fn GetFill(&self) -> windows_core::Result<super::d2d1::ID2D1Brush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFill)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn SetStroke<P0>(&self, brush: P0, strokewidth: f32, dashes: Option<&[f32]>, dashoffset: f32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d2d1::ID2D1Brush>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStroke)(windows_core::Interface::as_raw(self), brush.param().abi(), strokewidth, core::mem::transmute(dashes.map_or(core::ptr::null(), |slice| slice.as_ptr())), dashes.map_or(0, |slice| slice.len().try_into().unwrap()), dashoffset) }
    }
    pub unsafe fn GetStrokeDashesCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetStrokeDashesCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetStroke(&self, brush: *mut Option<super::d2d1::ID2D1Brush>, strokewidth: Option<*mut f32>, dashes: Option<&mut [f32]>, dashoffset: Option<*mut f32>) {
        unsafe {
            (windows_core::Interface::vtable(self).GetStroke)(windows_core::Interface::as_raw(self), core::mem::transmute(brush), strokewidth.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(dashes.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), dashes.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), dashoffset.unwrap_or(core::mem::zeroed()) as _);
        }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1SvgGlyphStyle_Vtbl {
    pub base__: super::d2d1::ID2D1Resource_Vtbl,
    pub SetFill: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFill: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub SetStroke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f32, *const f32, u32, f32) -> windows_core::HRESULT,
    pub GetStrokeDashesCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetStroke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut f32, *mut f32, u32, *mut f32),
}
#[cfg(feature = "Win32_d2d1")]
pub trait ID2D1SvgGlyphStyle_Impl: super::d2d1::ID2D1Resource_Impl {
    fn SetFill(&self, brush: windows_core::Ref<super::d2d1::ID2D1Brush>) -> windows_core::Result<()>;
    fn GetFill(&self, brush: windows_core::OutRef<super::d2d1::ID2D1Brush>);
    fn SetStroke(&self, brush: windows_core::Ref<super::d2d1::ID2D1Brush>, strokewidth: f32, dashes: *const f32, dashescount: u32, dashoffset: f32) -> windows_core::Result<()>;
    fn GetStrokeDashesCount(&self) -> u32;
    fn GetStroke(&self, brush: windows_core::OutRef<super::d2d1::ID2D1Brush>, strokewidth: *mut f32, dashes: *mut f32, dashescount: u32, dashoffset: *mut f32);
}
#[cfg(feature = "Win32_d2d1")]
impl ID2D1SvgGlyphStyle_Vtbl {
    pub const fn new<Identity: ID2D1SvgGlyphStyle_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetFill<Identity: ID2D1SvgGlyphStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, brush: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgGlyphStyle_Impl::SetFill(this, core::mem::transmute_copy(&brush)).into()
            }
        }
        unsafe extern "system" fn GetFill<Identity: ID2D1SvgGlyphStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, brush: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgGlyphStyle_Impl::GetFill(this, core::mem::transmute_copy(&brush));
            }
        }
        unsafe extern "system" fn SetStroke<Identity: ID2D1SvgGlyphStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, brush: *mut core::ffi::c_void, strokewidth: f32, dashes: *const f32, dashescount: u32, dashoffset: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgGlyphStyle_Impl::SetStroke(this, core::mem::transmute_copy(&brush), core::mem::transmute_copy(&strokewidth), core::mem::transmute_copy(&dashes), core::mem::transmute_copy(&dashescount), core::mem::transmute_copy(&dashoffset)).into()
            }
        }
        unsafe extern "system" fn GetStrokeDashesCount<Identity: ID2D1SvgGlyphStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgGlyphStyle_Impl::GetStrokeDashesCount(this)
            }
        }
        unsafe extern "system" fn GetStroke<Identity: ID2D1SvgGlyphStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, brush: *mut *mut core::ffi::c_void, strokewidth: *mut f32, dashes: *mut f32, dashescount: u32, dashoffset: *mut f32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgGlyphStyle_Impl::GetStroke(this, core::mem::transmute_copy(&brush), core::mem::transmute_copy(&strokewidth), core::mem::transmute_copy(&dashes), core::mem::transmute_copy(&dashescount), core::mem::transmute_copy(&dashoffset));
            }
        }
        Self {
            base__: super::d2d1::ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            SetFill: SetFill::<Identity, OFFSET>,
            GetFill: GetFill::<Identity, OFFSET>,
            SetStroke: SetStroke::<Identity, OFFSET>,
            GetStrokeDashesCount: GetStrokeDashesCount::<Identity, OFFSET>,
            GetStroke: GetStroke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1SvgGlyphStyle as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d2d1")]
impl windows_core::RuntimeName for ID2D1SvgGlyphStyle {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1TransformedImageSource, ID2D1TransformedImageSource_Vtbl, 0x7f1f79e5_2796_416c_8f55_700f911445e5);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1TransformedImageSource {
    type Target = super::d2d1::ID2D1Image;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1TransformedImageSource, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1::ID2D1Image);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1TransformedImageSource {
    pub unsafe fn GetSource(&self) -> windows_core::Result<ID2D1ImageSource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSource)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    #[cfg(feature = "Win32_d2d1_1")]
    pub unsafe fn GetProperties(&self, properties: *mut D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES) {
        unsafe {
            (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), properties as _);
        }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1TransformedImageSource_Vtbl {
    pub base__: super::d2d1::ID2D1Image_Vtbl,
    pub GetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    #[cfg(feature = "Win32_d2d1_1")]
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES),
    #[cfg(not(feature = "Win32_d2d1_1"))]
    GetProperties: usize,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
pub trait ID2D1TransformedImageSource_Impl: super::d2d1::ID2D1Image_Impl {
    fn GetSource(&self, imagesource: windows_core::OutRef<ID2D1ImageSource>);
    fn GetProperties(&self, properties: *mut D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES);
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
impl ID2D1TransformedImageSource_Vtbl {
    pub const fn new<Identity: ID2D1TransformedImageSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSource<Identity: ID2D1TransformedImageSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagesource: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1TransformedImageSource_Impl::GetSource(this, core::mem::transmute_copy(&imagesource));
            }
        }
        unsafe extern "system" fn GetProperties<Identity: ID2D1TransformedImageSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, properties: *mut D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1TransformedImageSource_Impl::GetProperties(this, core::mem::transmute_copy(&properties));
            }
        }
        Self {
            base__: super::d2d1::ID2D1Image_Vtbl::new::<Identity, OFFSET>(),
            GetSource: GetSource::<Identity, OFFSET>,
            GetProperties: GetProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1TransformedImageSource as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Image as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
impl windows_core::RuntimeName for ID2D1TransformedImageSource {}
