pub type D2D1_SVG_ASPECT_ALIGN = i32;
pub const D2D1_SVG_ASPECT_ALIGN_FORCE_DWORD: D2D1_SVG_ASPECT_ALIGN = -1;
pub const D2D1_SVG_ASPECT_ALIGN_NONE: D2D1_SVG_ASPECT_ALIGN = 0;
pub const D2D1_SVG_ASPECT_ALIGN_X_MAX_Y_MAX: D2D1_SVG_ASPECT_ALIGN = 9;
pub const D2D1_SVG_ASPECT_ALIGN_X_MAX_Y_MID: D2D1_SVG_ASPECT_ALIGN = 6;
pub const D2D1_SVG_ASPECT_ALIGN_X_MAX_Y_MIN: D2D1_SVG_ASPECT_ALIGN = 3;
pub const D2D1_SVG_ASPECT_ALIGN_X_MID_Y_MAX: D2D1_SVG_ASPECT_ALIGN = 8;
pub const D2D1_SVG_ASPECT_ALIGN_X_MID_Y_MID: D2D1_SVG_ASPECT_ALIGN = 5;
pub const D2D1_SVG_ASPECT_ALIGN_X_MID_Y_MIN: D2D1_SVG_ASPECT_ALIGN = 2;
pub const D2D1_SVG_ASPECT_ALIGN_X_MIN_Y_MAX: D2D1_SVG_ASPECT_ALIGN = 7;
pub const D2D1_SVG_ASPECT_ALIGN_X_MIN_Y_MID: D2D1_SVG_ASPECT_ALIGN = 4;
pub const D2D1_SVG_ASPECT_ALIGN_X_MIN_Y_MIN: D2D1_SVG_ASPECT_ALIGN = 1;
pub type D2D1_SVG_ASPECT_SCALING = i32;
pub const D2D1_SVG_ASPECT_SCALING_FORCE_DWORD: D2D1_SVG_ASPECT_SCALING = -1;
pub const D2D1_SVG_ASPECT_SCALING_MEET: D2D1_SVG_ASPECT_SCALING = 0;
pub const D2D1_SVG_ASPECT_SCALING_SLICE: D2D1_SVG_ASPECT_SCALING = 1;
pub type D2D1_SVG_ATTRIBUTE_POD_TYPE = i32;
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_COLOR: D2D1_SVG_ATTRIBUTE_POD_TYPE = 1;
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_DISPLAY: D2D1_SVG_ATTRIBUTE_POD_TYPE = 3;
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_EXTEND_MODE: D2D1_SVG_ATTRIBUTE_POD_TYPE = 10;
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_FILL_MODE: D2D1_SVG_ATTRIBUTE_POD_TYPE = 2;
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_FLOAT: D2D1_SVG_ATTRIBUTE_POD_TYPE = 0;
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_FORCE_DWORD: D2D1_SVG_ATTRIBUTE_POD_TYPE = -1;
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_LENGTH: D2D1_SVG_ATTRIBUTE_POD_TYPE = 13;
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_LINE_CAP: D2D1_SVG_ATTRIBUTE_POD_TYPE = 5;
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_LINE_JOIN: D2D1_SVG_ATTRIBUTE_POD_TYPE = 6;
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_MATRIX: D2D1_SVG_ATTRIBUTE_POD_TYPE = 8;
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_OVERFLOW: D2D1_SVG_ATTRIBUTE_POD_TYPE = 4;
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_PRESERVE_ASPECT_RATIO: D2D1_SVG_ATTRIBUTE_POD_TYPE = 11;
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_UNIT_TYPE: D2D1_SVG_ATTRIBUTE_POD_TYPE = 9;
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_VIEWBOX: D2D1_SVG_ATTRIBUTE_POD_TYPE = 12;
pub const D2D1_SVG_ATTRIBUTE_POD_TYPE_VISIBILITY: D2D1_SVG_ATTRIBUTE_POD_TYPE = 7;
pub type D2D1_SVG_ATTRIBUTE_STRING_TYPE = i32;
pub const D2D1_SVG_ATTRIBUTE_STRING_TYPE_FORCE_DWORD: D2D1_SVG_ATTRIBUTE_STRING_TYPE = -1;
pub const D2D1_SVG_ATTRIBUTE_STRING_TYPE_ID: D2D1_SVG_ATTRIBUTE_STRING_TYPE = 1;
pub const D2D1_SVG_ATTRIBUTE_STRING_TYPE_SVG: D2D1_SVG_ATTRIBUTE_STRING_TYPE = 0;
pub type D2D1_SVG_DISPLAY = i32;
pub const D2D1_SVG_DISPLAY_FORCE_DWORD: D2D1_SVG_DISPLAY = -1;
pub const D2D1_SVG_DISPLAY_INLINE: D2D1_SVG_DISPLAY = 0;
pub const D2D1_SVG_DISPLAY_NONE: D2D1_SVG_DISPLAY = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_SVG_LENGTH {
    pub value: f32,
    pub units: D2D1_SVG_LENGTH_UNITS,
}
pub type D2D1_SVG_LENGTH_UNITS = i32;
pub const D2D1_SVG_LENGTH_UNITS_FORCE_DWORD: D2D1_SVG_LENGTH_UNITS = -1;
pub const D2D1_SVG_LENGTH_UNITS_NUMBER: D2D1_SVG_LENGTH_UNITS = 0;
pub const D2D1_SVG_LENGTH_UNITS_PERCENTAGE: D2D1_SVG_LENGTH_UNITS = 1;
pub type D2D1_SVG_LINE_CAP = i32;
pub const D2D1_SVG_LINE_CAP_BUTT: D2D1_SVG_LINE_CAP = 0;
pub const D2D1_SVG_LINE_CAP_FORCE_DWORD: D2D1_SVG_LINE_CAP = -1;
pub const D2D1_SVG_LINE_CAP_ROUND: D2D1_SVG_LINE_CAP = 2;
pub const D2D1_SVG_LINE_CAP_SQUARE: D2D1_SVG_LINE_CAP = 1;
pub type D2D1_SVG_LINE_JOIN = i32;
pub const D2D1_SVG_LINE_JOIN_BEVEL: D2D1_SVG_LINE_JOIN = 1;
pub const D2D1_SVG_LINE_JOIN_FORCE_DWORD: D2D1_SVG_LINE_JOIN = -1;
pub const D2D1_SVG_LINE_JOIN_MITER: D2D1_SVG_LINE_JOIN = 3;
pub const D2D1_SVG_LINE_JOIN_ROUND: D2D1_SVG_LINE_JOIN = 2;
pub type D2D1_SVG_OVERFLOW = i32;
pub const D2D1_SVG_OVERFLOW_FORCE_DWORD: D2D1_SVG_OVERFLOW = -1;
pub const D2D1_SVG_OVERFLOW_HIDDEN: D2D1_SVG_OVERFLOW = 1;
pub const D2D1_SVG_OVERFLOW_VISIBLE: D2D1_SVG_OVERFLOW = 0;
pub type D2D1_SVG_PAINT_TYPE = i32;
pub const D2D1_SVG_PAINT_TYPE_COLOR: D2D1_SVG_PAINT_TYPE = 1;
pub const D2D1_SVG_PAINT_TYPE_CURRENT_COLOR: D2D1_SVG_PAINT_TYPE = 2;
pub const D2D1_SVG_PAINT_TYPE_FORCE_DWORD: D2D1_SVG_PAINT_TYPE = -1;
pub const D2D1_SVG_PAINT_TYPE_NONE: D2D1_SVG_PAINT_TYPE = 0;
pub const D2D1_SVG_PAINT_TYPE_URI: D2D1_SVG_PAINT_TYPE = 3;
pub const D2D1_SVG_PAINT_TYPE_URI_COLOR: D2D1_SVG_PAINT_TYPE = 5;
pub const D2D1_SVG_PAINT_TYPE_URI_CURRENT_COLOR: D2D1_SVG_PAINT_TYPE = 6;
pub const D2D1_SVG_PAINT_TYPE_URI_NONE: D2D1_SVG_PAINT_TYPE = 4;
pub type D2D1_SVG_PATH_COMMAND = i32;
pub const D2D1_SVG_PATH_COMMAND_ARC_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 9;
pub const D2D1_SVG_PATH_COMMAND_ARC_RELATIVE: D2D1_SVG_PATH_COMMAND = 10;
pub const D2D1_SVG_PATH_COMMAND_CLOSE_PATH: D2D1_SVG_PATH_COMMAND = 0;
pub const D2D1_SVG_PATH_COMMAND_CUBIC_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 5;
pub const D2D1_SVG_PATH_COMMAND_CUBIC_RELATIVE: D2D1_SVG_PATH_COMMAND = 6;
pub const D2D1_SVG_PATH_COMMAND_CUBIC_SMOOTH_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 15;
pub const D2D1_SVG_PATH_COMMAND_CUBIC_SMOOTH_RELATIVE: D2D1_SVG_PATH_COMMAND = 16;
pub const D2D1_SVG_PATH_COMMAND_FORCE_DWORD: D2D1_SVG_PATH_COMMAND = -1;
pub const D2D1_SVG_PATH_COMMAND_HORIZONTAL_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 11;
pub const D2D1_SVG_PATH_COMMAND_HORIZONTAL_RELATIVE: D2D1_SVG_PATH_COMMAND = 12;
pub const D2D1_SVG_PATH_COMMAND_LINE_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 3;
pub const D2D1_SVG_PATH_COMMAND_LINE_RELATIVE: D2D1_SVG_PATH_COMMAND = 4;
pub const D2D1_SVG_PATH_COMMAND_MOVE_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 1;
pub const D2D1_SVG_PATH_COMMAND_MOVE_RELATIVE: D2D1_SVG_PATH_COMMAND = 2;
pub const D2D1_SVG_PATH_COMMAND_QUADRADIC_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 7;
pub const D2D1_SVG_PATH_COMMAND_QUADRADIC_RELATIVE: D2D1_SVG_PATH_COMMAND = 8;
pub const D2D1_SVG_PATH_COMMAND_QUADRADIC_SMOOTH_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 17;
pub const D2D1_SVG_PATH_COMMAND_QUADRADIC_SMOOTH_RELATIVE: D2D1_SVG_PATH_COMMAND = 18;
pub const D2D1_SVG_PATH_COMMAND_VERTICAL_ABSOLUTE: D2D1_SVG_PATH_COMMAND = 13;
pub const D2D1_SVG_PATH_COMMAND_VERTICAL_RELATIVE: D2D1_SVG_PATH_COMMAND = 14;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_SVG_PRESERVE_ASPECT_RATIO {
    pub defer: windows_core::BOOL,
    pub align: D2D1_SVG_ASPECT_ALIGN,
    pub meetOrSlice: D2D1_SVG_ASPECT_SCALING,
}
pub type D2D1_SVG_UNIT_TYPE = i32;
pub const D2D1_SVG_UNIT_TYPE_FORCE_DWORD: D2D1_SVG_UNIT_TYPE = -1;
pub const D2D1_SVG_UNIT_TYPE_OBJECT_BOUNDING_BOX: D2D1_SVG_UNIT_TYPE = 1;
pub const D2D1_SVG_UNIT_TYPE_USER_SPACE_ON_USE: D2D1_SVG_UNIT_TYPE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_SVG_VIEWBOX {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
pub type D2D1_SVG_VISIBILITY = i32;
pub const D2D1_SVG_VISIBILITY_FORCE_DWORD: D2D1_SVG_VISIBILITY = -1;
pub const D2D1_SVG_VISIBILITY_HIDDEN: D2D1_SVG_VISIBILITY = 1;
pub const D2D1_SVG_VISIBILITY_VISIBLE: D2D1_SVG_VISIBILITY = 0;
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1SvgAttribute, ID2D1SvgAttribute_Vtbl, 0xc9cdb0dd_f8c9_4e70_b7c2_301c80292c5e);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1SvgAttribute {
    type Target = super::d2d1::ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1SvgAttribute, windows_core::IUnknown, super::d2d1::ID2D1Resource);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1SvgAttribute {
    pub unsafe fn GetElement(&self) -> windows_core::Result<ID2D1SvgElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1SvgAttribute_Vtbl {
    pub base__: super::d2d1::ID2D1Resource_Vtbl,
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_d2d1")]
pub trait ID2D1SvgAttribute_Impl: super::d2d1::ID2D1Resource_Impl {
    fn GetElement(&self, element: windows_core::OutRef<ID2D1SvgElement>);
    fn Clone(&self) -> windows_core::Result<ID2D1SvgAttribute>;
}
#[cfg(feature = "Win32_d2d1")]
impl ID2D1SvgAttribute_Vtbl {
    pub const fn new<Identity: ID2D1SvgAttribute_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetElement<Identity: ID2D1SvgAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgAttribute_Impl::GetElement(this, core::mem::transmute_copy(&element));
            }
        }
        unsafe extern "system" fn Clone<Identity: ID2D1SvgAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attribute: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1SvgAttribute_Impl::Clone(this) {
                    Ok(ok__) => {
                        attribute.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::d2d1::ID2D1Resource_Vtbl::new::<Identity, OFFSET>(), GetElement: GetElement::<Identity, OFFSET>, Clone: Clone::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1SvgAttribute as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d2d1")]
impl windows_core::RuntimeName for ID2D1SvgAttribute {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1SvgDocument, ID2D1SvgDocument_Vtbl, 0x86b88e4d_afa4_4d7b_88e4_68a51c4a0aec);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1SvgDocument {
    type Target = super::d2d1::ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1SvgDocument, windows_core::IUnknown, super::d2d1::ID2D1Resource);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1SvgDocument {
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn SetViewportSize(&self, viewportsize: super::dcommon::D2D_SIZE_F) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetViewportSize)(windows_core::Interface::as_raw(self), core::mem::transmute(viewportsize)) }
    }
    #[cfg(feature = "Win32_dcommon")]
    pub unsafe fn GetViewportSize(&self) -> super::dcommon::D2D_SIZE_F {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetViewportSize)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn SetRoot<P0>(&self, root: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ID2D1SvgElement>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRoot)(windows_core::Interface::as_raw(self), root.param().abi()) }
    }
    pub unsafe fn GetRoot(&self) -> windows_core::Result<ID2D1SvgElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRoot)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn FindElementById<P0>(&self, id: P0) -> windows_core::Result<ID2D1SvgElement>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindElementById)(windows_core::Interface::as_raw(self), id.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn Serialize<P0, P1>(&self, outputxmlstream: P0, subtree: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
        P1: windows_core::Param<ID2D1SvgElement>,
    {
        unsafe { (windows_core::Interface::vtable(self).Serialize)(windows_core::Interface::as_raw(self), outputxmlstream.param().abi(), subtree.param().abi()) }
    }
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn Deserialize<P0>(&self, inputxmlstream: P0) -> windows_core::Result<ID2D1SvgElement>
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Deserialize)(windows_core::Interface::as_raw(self), inputxmlstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub unsafe fn CreatePaint<P2>(&self, painttype: D2D1_SVG_PAINT_TYPE, color: Option<*const super::d2dbasetypes::D2D_COLOR_F>, id: P2) -> windows_core::Result<ID2D1SvgPaint>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePaint)(windows_core::Interface::as_raw(self), painttype, color.unwrap_or(core::mem::zeroed()) as _, id.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateStrokeDashArray(&self, dashes: Option<&[D2D1_SVG_LENGTH]>) -> windows_core::Result<ID2D1SvgStrokeDashArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStrokeDashArray)(windows_core::Interface::as_raw(self), core::mem::transmute(dashes.map_or(core::ptr::null(), |slice| slice.as_ptr())), dashes.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreatePointCollection(&self, points: Option<&[windows_numerics::Vector2]>) -> windows_core::Result<ID2D1SvgPointCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePointCollection)(windows_core::Interface::as_raw(self), core::mem::transmute(points.map_or(core::ptr::null(), |slice| slice.as_ptr())), points.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreatePathData(&self, segmentdata: Option<&[f32]>, commands: Option<&[D2D1_SVG_PATH_COMMAND]>) -> windows_core::Result<ID2D1SvgPathData> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePathData)(windows_core::Interface::as_raw(self), core::mem::transmute(segmentdata.map_or(core::ptr::null(), |slice| slice.as_ptr())), segmentdata.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(commands.map_or(core::ptr::null(), |slice| slice.as_ptr())), commands.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1SvgDocument_Vtbl {
    pub base__: super::d2d1::ID2D1Resource_Vtbl,
    #[cfg(feature = "Win32_dcommon")]
    pub SetViewportSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::dcommon::D2D_SIZE_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dcommon"))]
    SetViewportSize: usize,
    #[cfg(feature = "Win32_dcommon")]
    pub GetViewportSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::dcommon::D2D_SIZE_F),
    #[cfg(not(feature = "Win32_dcommon"))]
    GetViewportSize: usize,
    pub SetRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub FindElementById: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_objidlbase")]
    pub Serialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    Serialize: usize,
    #[cfg(feature = "Win32_objidlbase")]
    pub Deserialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    Deserialize: usize,
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub CreatePaint: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_SVG_PAINT_TYPE, *const super::d2dbasetypes::D2D_COLOR_F, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype")))]
    CreatePaint: usize,
    pub CreateStrokeDashArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_SVG_LENGTH, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePointCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Vector2, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePathData: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32, u32, *const D2D1_SVG_PATH_COMMAND, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dxgitype", feature = "Win32_objidlbase"))]
pub trait ID2D1SvgDocument_Impl: super::d2d1::ID2D1Resource_Impl {
    fn SetViewportSize(&self, viewportsize: &super::dcommon::D2D_SIZE_F) -> windows_core::Result<()>;
    fn GetViewportSize(&self) -> super::dcommon::D2D_SIZE_F;
    fn SetRoot(&self, root: windows_core::Ref<ID2D1SvgElement>) -> windows_core::Result<()>;
    fn GetRoot(&self, root: windows_core::OutRef<ID2D1SvgElement>);
    fn FindElementById(&self, id: &windows_core::PCWSTR) -> windows_core::Result<ID2D1SvgElement>;
    fn Serialize(&self, outputxmlstream: windows_core::Ref<super::objidlbase::IStream>, subtree: windows_core::Ref<ID2D1SvgElement>) -> windows_core::Result<()>;
    fn Deserialize(&self, inputxmlstream: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<ID2D1SvgElement>;
    fn CreatePaint(&self, painttype: D2D1_SVG_PAINT_TYPE, color: *const super::d2dbasetypes::D2D_COLOR_F, id: &windows_core::PCWSTR) -> windows_core::Result<ID2D1SvgPaint>;
    fn CreateStrokeDashArray(&self, dashes: *const D2D1_SVG_LENGTH, dashescount: u32) -> windows_core::Result<ID2D1SvgStrokeDashArray>;
    fn CreatePointCollection(&self, points: *const windows_numerics::Vector2, pointscount: u32) -> windows_core::Result<ID2D1SvgPointCollection>;
    fn CreatePathData(&self, segmentdata: *const f32, segmentdatacount: u32, commands: *const D2D1_SVG_PATH_COMMAND, commandscount: u32) -> windows_core::Result<ID2D1SvgPathData>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dxgitype", feature = "Win32_objidlbase"))]
impl ID2D1SvgDocument_Vtbl {
    pub const fn new<Identity: ID2D1SvgDocument_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetViewportSize<Identity: ID2D1SvgDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewportsize: super::dcommon::D2D_SIZE_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgDocument_Impl::SetViewportSize(this, core::mem::transmute(&viewportsize)).into()
            }
        }
        unsafe extern "system" fn GetViewportSize<Identity: ID2D1SvgDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::dcommon::D2D_SIZE_F) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                *result__ = ID2D1SvgDocument_Impl::GetViewportSize(this);
            }
        }
        unsafe extern "system" fn SetRoot<Identity: ID2D1SvgDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, root: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgDocument_Impl::SetRoot(this, core::mem::transmute_copy(&root)).into()
            }
        }
        unsafe extern "system" fn GetRoot<Identity: ID2D1SvgDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, root: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgDocument_Impl::GetRoot(this, core::mem::transmute_copy(&root));
            }
        }
        unsafe extern "system" fn FindElementById<Identity: ID2D1SvgDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: windows_core::PCWSTR, svgelement: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1SvgDocument_Impl::FindElementById(this, core::mem::transmute(&id)) {
                    Ok(ok__) => {
                        svgelement.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Serialize<Identity: ID2D1SvgDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputxmlstream: *mut core::ffi::c_void, subtree: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgDocument_Impl::Serialize(this, core::mem::transmute_copy(&outputxmlstream), core::mem::transmute_copy(&subtree)).into()
            }
        }
        unsafe extern "system" fn Deserialize<Identity: ID2D1SvgDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputxmlstream: *mut core::ffi::c_void, subtree: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1SvgDocument_Impl::Deserialize(this, core::mem::transmute_copy(&inputxmlstream)) {
                    Ok(ok__) => {
                        subtree.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePaint<Identity: ID2D1SvgDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, painttype: D2D1_SVG_PAINT_TYPE, color: *const super::d2dbasetypes::D2D_COLOR_F, id: windows_core::PCWSTR, paint: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1SvgDocument_Impl::CreatePaint(this, core::mem::transmute_copy(&painttype), core::mem::transmute_copy(&color), core::mem::transmute(&id)) {
                    Ok(ok__) => {
                        paint.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateStrokeDashArray<Identity: ID2D1SvgDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dashes: *const D2D1_SVG_LENGTH, dashescount: u32, strokedasharray: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1SvgDocument_Impl::CreateStrokeDashArray(this, core::mem::transmute_copy(&dashes), core::mem::transmute_copy(&dashescount)) {
                    Ok(ok__) => {
                        strokedasharray.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePointCollection<Identity: ID2D1SvgDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, points: *const windows_numerics::Vector2, pointscount: u32, pointcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1SvgDocument_Impl::CreatePointCollection(this, core::mem::transmute_copy(&points), core::mem::transmute_copy(&pointscount)) {
                    Ok(ok__) => {
                        pointcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePathData<Identity: ID2D1SvgDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, segmentdata: *const f32, segmentdatacount: u32, commands: *const D2D1_SVG_PATH_COMMAND, commandscount: u32, pathdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1SvgDocument_Impl::CreatePathData(this, core::mem::transmute_copy(&segmentdata), core::mem::transmute_copy(&segmentdatacount), core::mem::transmute_copy(&commands), core::mem::transmute_copy(&commandscount)) {
                    Ok(ok__) => {
                        pathdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::d2d1::ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            SetViewportSize: SetViewportSize::<Identity, OFFSET>,
            GetViewportSize: GetViewportSize::<Identity, OFFSET>,
            SetRoot: SetRoot::<Identity, OFFSET>,
            GetRoot: GetRoot::<Identity, OFFSET>,
            FindElementById: FindElementById::<Identity, OFFSET>,
            Serialize: Serialize::<Identity, OFFSET>,
            Deserialize: Deserialize::<Identity, OFFSET>,
            CreatePaint: CreatePaint::<Identity, OFFSET>,
            CreateStrokeDashArray: CreateStrokeDashArray::<Identity, OFFSET>,
            CreatePointCollection: CreatePointCollection::<Identity, OFFSET>,
            CreatePathData: CreatePathData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1SvgDocument as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dxgitype", feature = "Win32_objidlbase"))]
impl windows_core::RuntimeName for ID2D1SvgDocument {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1SvgElement, ID2D1SvgElement_Vtbl, 0xac7b67a6_183e_49c1_a823_0ebe40b0db29);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1SvgElement {
    type Target = super::d2d1::ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1SvgElement, windows_core::IUnknown, super::d2d1::ID2D1Resource);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1SvgElement {
    pub unsafe fn GetDocument(&self) -> windows_core::Result<ID2D1SvgDocument> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocument)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn GetTagName(&self, name: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTagName)(windows_core::Interface::as_raw(self), core::mem::transmute(name.as_ptr()), name.len().try_into().unwrap()) }
    }
    pub unsafe fn GetTagNameLength(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetTagNameLength)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsTextContent(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsTextContent)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetParent(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParent)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn HasChildren(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).HasChildren)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetTopWindow(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTopWindow)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn GetLastChild(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastChild)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn GetPreviousChild<P0>(&self, referencechild: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPreviousChild)(windows_core::Interface::as_raw(self), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetNextChild<P0>(&self, referencechild: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNextChild)(windows_core::Interface::as_raw(self), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn InsertChildBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertChildBefore)(windows_core::Interface::as_raw(self), newchild.param().abi(), referencechild.param().abi()) }
    }
    pub unsafe fn AppendChild<P0>(&self, newchild: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).AppendChild)(windows_core::Interface::as_raw(self), newchild.param().abi()) }
    }
    pub unsafe fn ReplaceChild<P0, P1>(&self, newchild: P0, oldchild: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReplaceChild)(windows_core::Interface::as_raw(self), newchild.param().abi(), oldchild.param().abi()) }
    }
    pub unsafe fn RemoveChild<P0>(&self, oldchild: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveChild)(windows_core::Interface::as_raw(self), oldchild.param().abi()) }
    }
    pub unsafe fn CreateChild<P0>(&self, tagname: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateChild)(windows_core::Interface::as_raw(self), tagname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsAttributeSpecified<P0>(&self, name: P0, inherited: Option<*mut windows_core::BOOL>) -> windows_core::BOOL
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsAttributeSpecified)(windows_core::Interface::as_raw(self), name.param().abi(), inherited.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetSpecifiedAttributeCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetSpecifiedAttributeCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetSpecifiedAttributeName(&self, index: u32, name: &mut [u16], inherited: Option<*mut windows_core::BOOL>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSpecifiedAttributeName)(windows_core::Interface::as_raw(self), index, core::mem::transmute(name.as_ptr()), name.len().try_into().unwrap(), inherited.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetSpecifiedAttributeNameLength(&self, index: u32, namelength: *mut u32, inherited: Option<*mut windows_core::BOOL>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSpecifiedAttributeNameLength)(windows_core::Interface::as_raw(self), index, namelength as _, inherited.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn RemoveAttribute<P0>(&self, name: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveAttribute)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn SetTextValue(&self, name: &[u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTextValue)(windows_core::Interface::as_raw(self), core::mem::transmute(name.as_ptr()), name.len().try_into().unwrap()) }
    }
    pub unsafe fn GetTextValue(&self, name: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTextValue)(windows_core::Interface::as_raw(self), core::mem::transmute(name.as_ptr()), name.len().try_into().unwrap()) }
    }
    pub unsafe fn GetTextValueLength(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetTextValueLength)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetAttributeValue<P0, P2>(&self, name: P0, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, value: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAttributeValue)(windows_core::Interface::as_raw(self), name.param().abi(), r#type, value.param().abi()) }
    }
    pub unsafe fn GetAttributeValue<P0>(&self, name: P0, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, value: &mut [u16]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAttributeValue)(windows_core::Interface::as_raw(self), name.param().abi(), r#type, core::mem::transmute(value.as_ptr()), value.len().try_into().unwrap()) }
    }
    pub unsafe fn GetAttributeValueLength<P0>(&self, name: P0, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttributeValueLength)(windows_core::Interface::as_raw(self), name.param().abi(), r#type, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAttributeValue2<P0>(&self, name: P0, r#type: D2D1_SVG_ATTRIBUTE_POD_TYPE, value: *const core::ffi::c_void, valuesizeinbytes: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAttributeValue2)(windows_core::Interface::as_raw(self), name.param().abi(), r#type, value, valuesizeinbytes) }
    }
    pub unsafe fn GetAttributeValue2<P0>(&self, name: P0, r#type: D2D1_SVG_ATTRIBUTE_POD_TYPE, value: *mut core::ffi::c_void, valuesizeinbytes: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetAttributeValue2)(windows_core::Interface::as_raw(self), name.param().abi(), r#type, value as _, valuesizeinbytes) }
    }
    pub unsafe fn SetAttributeValue3<P0, P1>(&self, name: P0, value: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<ID2D1SvgAttribute>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAttributeValue3)(windows_core::Interface::as_raw(self), name.param().abi(), value.param().abi()) }
    }
    pub unsafe fn GetAttributeValue3<P0, T>(&self, name: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetAttributeValue3)(windows_core::Interface::as_raw(self), name.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1SvgElement_Vtbl {
    pub base__: super::d2d1::ID2D1Resource_Vtbl,
    pub GetDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub GetTagName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetTagNameLength: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub IsTextContent: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetParent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub HasChildren: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetTopWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub GetLastChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub GetPreviousChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNextChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertChildBefore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppendChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReplaceChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateChild: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsAttributeSpecified: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::BOOL,
    pub GetSpecifiedAttributeCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetSpecifiedAttributeName: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetSpecifiedAttributeNameLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub RemoveAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetTextValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, u32) -> windows_core::HRESULT,
    pub GetTextValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetTextValueLength: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub SetAttributeValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, D2D1_SVG_ATTRIBUTE_STRING_TYPE, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetAttributeValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, D2D1_SVG_ATTRIBUTE_STRING_TYPE, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetAttributeValueLength: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, D2D1_SVG_ATTRIBUTE_STRING_TYPE, *mut u32) -> windows_core::HRESULT,
    pub SetAttributeValue2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, D2D1_SVG_ATTRIBUTE_POD_TYPE, *const core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetAttributeValue2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, D2D1_SVG_ATTRIBUTE_POD_TYPE, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetAttributeValue3: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAttributeValue3: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_d2d1")]
pub trait ID2D1SvgElement_Impl: super::d2d1::ID2D1Resource_Impl {
    fn GetDocument(&self, document: windows_core::OutRef<ID2D1SvgDocument>);
    fn GetTagName(&self, name: windows_core::PWSTR, namecount: u32) -> windows_core::Result<()>;
    fn GetTagNameLength(&self) -> u32;
    fn IsTextContent(&self) -> windows_core::BOOL;
    fn GetParent(&self, parent: windows_core::OutRef<ID2D1SvgElement>);
    fn HasChildren(&self) -> windows_core::BOOL;
    fn GetTopWindow(&self, child: windows_core::OutRef<ID2D1SvgElement>);
    fn GetLastChild(&self, child: windows_core::OutRef<ID2D1SvgElement>);
    fn GetPreviousChild(&self, referencechild: windows_core::Ref<ID2D1SvgElement>) -> windows_core::Result<ID2D1SvgElement>;
    fn GetNextChild(&self, referencechild: windows_core::Ref<ID2D1SvgElement>) -> windows_core::Result<ID2D1SvgElement>;
    fn InsertChildBefore(&self, newchild: windows_core::Ref<ID2D1SvgElement>, referencechild: windows_core::Ref<ID2D1SvgElement>) -> windows_core::Result<()>;
    fn AppendChild(&self, newchild: windows_core::Ref<ID2D1SvgElement>) -> windows_core::Result<()>;
    fn ReplaceChild(&self, newchild: windows_core::Ref<ID2D1SvgElement>, oldchild: windows_core::Ref<ID2D1SvgElement>) -> windows_core::Result<()>;
    fn RemoveChild(&self, oldchild: windows_core::Ref<ID2D1SvgElement>) -> windows_core::Result<()>;
    fn CreateChild(&self, tagname: &windows_core::PCWSTR) -> windows_core::Result<ID2D1SvgElement>;
    fn IsAttributeSpecified(&self, name: &windows_core::PCWSTR, inherited: *mut windows_core::BOOL) -> windows_core::BOOL;
    fn GetSpecifiedAttributeCount(&self) -> u32;
    fn GetSpecifiedAttributeName(&self, index: u32, name: windows_core::PWSTR, namecount: u32, inherited: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetSpecifiedAttributeNameLength(&self, index: u32, namelength: *mut u32, inherited: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn RemoveAttribute(&self, name: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetTextValue(&self, name: *const u16, namecount: u32) -> windows_core::Result<()>;
    fn GetTextValue(&self, name: windows_core::PWSTR, namecount: u32) -> windows_core::Result<()>;
    fn GetTextValueLength(&self) -> u32;
    fn SetAttributeValue(&self, name: &windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, value: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetAttributeValue(&self, name: &windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, value: windows_core::PWSTR, valuecount: u32) -> windows_core::Result<()>;
    fn GetAttributeValueLength(&self, name: &windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE) -> windows_core::Result<u32>;
    fn SetAttributeValue2(&self, name: &windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_POD_TYPE, value: *const core::ffi::c_void, valuesizeinbytes: u32) -> windows_core::Result<()>;
    fn GetAttributeValue2(&self, name: &windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_POD_TYPE, value: *mut core::ffi::c_void, valuesizeinbytes: u32) -> windows_core::Result<()>;
    fn SetAttributeValue3(&self, name: &windows_core::PCWSTR, value: windows_core::Ref<ID2D1SvgAttribute>) -> windows_core::Result<()>;
    fn GetAttributeValue3(&self, name: &windows_core::PCWSTR, riid: *const windows_core::GUID, value: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_d2d1")]
impl ID2D1SvgElement_Vtbl {
    pub const fn new<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDocument<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, document: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgElement_Impl::GetDocument(this, core::mem::transmute_copy(&document));
            }
        }
        unsafe extern "system" fn GetTagName<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PWSTR, namecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgElement_Impl::GetTagName(this, core::mem::transmute_copy(&name), core::mem::transmute_copy(&namecount)).into()
            }
        }
        unsafe extern "system" fn GetTagNameLength<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgElement_Impl::GetTagNameLength(this)
            }
        }
        unsafe extern "system" fn IsTextContent<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgElement_Impl::IsTextContent(this)
            }
        }
        unsafe extern "system" fn GetParent<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parent: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgElement_Impl::GetParent(this, core::mem::transmute_copy(&parent));
            }
        }
        unsafe extern "system" fn HasChildren<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgElement_Impl::HasChildren(this)
            }
        }
        unsafe extern "system" fn GetTopWindow<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, child: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgElement_Impl::GetTopWindow(this, core::mem::transmute_copy(&child));
            }
        }
        unsafe extern "system" fn GetLastChild<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, child: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgElement_Impl::GetLastChild(this, core::mem::transmute_copy(&child));
            }
        }
        unsafe extern "system" fn GetPreviousChild<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, referencechild: *mut core::ffi::c_void, previouschild: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1SvgElement_Impl::GetPreviousChild(this, core::mem::transmute_copy(&referencechild)) {
                    Ok(ok__) => {
                        previouschild.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNextChild<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, referencechild: *mut core::ffi::c_void, nextchild: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1SvgElement_Impl::GetNextChild(this, core::mem::transmute_copy(&referencechild)) {
                    Ok(ok__) => {
                        nextchild.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertChildBefore<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newchild: *mut core::ffi::c_void, referencechild: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgElement_Impl::InsertChildBefore(this, core::mem::transmute_copy(&newchild), core::mem::transmute_copy(&referencechild)).into()
            }
        }
        unsafe extern "system" fn AppendChild<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newchild: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgElement_Impl::AppendChild(this, core::mem::transmute_copy(&newchild)).into()
            }
        }
        unsafe extern "system" fn ReplaceChild<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newchild: *mut core::ffi::c_void, oldchild: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgElement_Impl::ReplaceChild(this, core::mem::transmute_copy(&newchild), core::mem::transmute_copy(&oldchild)).into()
            }
        }
        unsafe extern "system" fn RemoveChild<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldchild: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgElement_Impl::RemoveChild(this, core::mem::transmute_copy(&oldchild)).into()
            }
        }
        unsafe extern "system" fn CreateChild<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tagname: windows_core::PCWSTR, newchild: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1SvgElement_Impl::CreateChild(this, core::mem::transmute(&tagname)) {
                    Ok(ok__) => {
                        newchild.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsAttributeSpecified<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, inherited: *mut windows_core::BOOL) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgElement_Impl::IsAttributeSpecified(this, core::mem::transmute(&name), core::mem::transmute_copy(&inherited))
            }
        }
        unsafe extern "system" fn GetSpecifiedAttributeCount<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgElement_Impl::GetSpecifiedAttributeCount(this)
            }
        }
        unsafe extern "system" fn GetSpecifiedAttributeName<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, name: windows_core::PWSTR, namecount: u32, inherited: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgElement_Impl::GetSpecifiedAttributeName(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&name), core::mem::transmute_copy(&namecount), core::mem::transmute_copy(&inherited)).into()
            }
        }
        unsafe extern "system" fn GetSpecifiedAttributeNameLength<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, namelength: *mut u32, inherited: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgElement_Impl::GetSpecifiedAttributeNameLength(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&namelength), core::mem::transmute_copy(&inherited)).into()
            }
        }
        unsafe extern "system" fn RemoveAttribute<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgElement_Impl::RemoveAttribute(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn SetTextValue<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *const u16, namecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgElement_Impl::SetTextValue(this, core::mem::transmute_copy(&name), core::mem::transmute_copy(&namecount)).into()
            }
        }
        unsafe extern "system" fn GetTextValue<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PWSTR, namecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgElement_Impl::GetTextValue(this, core::mem::transmute_copy(&name), core::mem::transmute_copy(&namecount)).into()
            }
        }
        unsafe extern "system" fn GetTextValueLength<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgElement_Impl::GetTextValueLength(this)
            }
        }
        unsafe extern "system" fn SetAttributeValue<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, value: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgElement_Impl::SetAttributeValue(this, core::mem::transmute(&name), core::mem::transmute_copy(&r#type), core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn GetAttributeValue<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, value: windows_core::PWSTR, valuecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgElement_Impl::GetAttributeValue(this, core::mem::transmute(&name), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value), core::mem::transmute_copy(&valuecount)).into()
            }
        }
        unsafe extern "system" fn GetAttributeValueLength<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_STRING_TYPE, valuelength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1SvgElement_Impl::GetAttributeValueLength(this, core::mem::transmute(&name), core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        valuelength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAttributeValue2<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_POD_TYPE, value: *const core::ffi::c_void, valuesizeinbytes: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgElement_Impl::SetAttributeValue2(this, core::mem::transmute(&name), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value), core::mem::transmute_copy(&valuesizeinbytes)).into()
            }
        }
        unsafe extern "system" fn GetAttributeValue2<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, r#type: D2D1_SVG_ATTRIBUTE_POD_TYPE, value: *mut core::ffi::c_void, valuesizeinbytes: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgElement_Impl::GetAttributeValue2(this, core::mem::transmute(&name), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&value), core::mem::transmute_copy(&valuesizeinbytes)).into()
            }
        }
        unsafe extern "system" fn SetAttributeValue3<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgElement_Impl::SetAttributeValue3(this, core::mem::transmute(&name), core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetAttributeValue3<Identity: ID2D1SvgElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, riid: *const windows_core::GUID, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgElement_Impl::GetAttributeValue3(this, core::mem::transmute(&name), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: super::d2d1::ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            GetDocument: GetDocument::<Identity, OFFSET>,
            GetTagName: GetTagName::<Identity, OFFSET>,
            GetTagNameLength: GetTagNameLength::<Identity, OFFSET>,
            IsTextContent: IsTextContent::<Identity, OFFSET>,
            GetParent: GetParent::<Identity, OFFSET>,
            HasChildren: HasChildren::<Identity, OFFSET>,
            GetTopWindow: GetTopWindow::<Identity, OFFSET>,
            GetLastChild: GetLastChild::<Identity, OFFSET>,
            GetPreviousChild: GetPreviousChild::<Identity, OFFSET>,
            GetNextChild: GetNextChild::<Identity, OFFSET>,
            InsertChildBefore: InsertChildBefore::<Identity, OFFSET>,
            AppendChild: AppendChild::<Identity, OFFSET>,
            ReplaceChild: ReplaceChild::<Identity, OFFSET>,
            RemoveChild: RemoveChild::<Identity, OFFSET>,
            CreateChild: CreateChild::<Identity, OFFSET>,
            IsAttributeSpecified: IsAttributeSpecified::<Identity, OFFSET>,
            GetSpecifiedAttributeCount: GetSpecifiedAttributeCount::<Identity, OFFSET>,
            GetSpecifiedAttributeName: GetSpecifiedAttributeName::<Identity, OFFSET>,
            GetSpecifiedAttributeNameLength: GetSpecifiedAttributeNameLength::<Identity, OFFSET>,
            RemoveAttribute: RemoveAttribute::<Identity, OFFSET>,
            SetTextValue: SetTextValue::<Identity, OFFSET>,
            GetTextValue: GetTextValue::<Identity, OFFSET>,
            GetTextValueLength: GetTextValueLength::<Identity, OFFSET>,
            SetAttributeValue: SetAttributeValue::<Identity, OFFSET>,
            GetAttributeValue: GetAttributeValue::<Identity, OFFSET>,
            GetAttributeValueLength: GetAttributeValueLength::<Identity, OFFSET>,
            SetAttributeValue2: SetAttributeValue2::<Identity, OFFSET>,
            GetAttributeValue2: GetAttributeValue2::<Identity, OFFSET>,
            SetAttributeValue3: SetAttributeValue3::<Identity, OFFSET>,
            GetAttributeValue3: GetAttributeValue3::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1SvgElement as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d2d1")]
impl windows_core::RuntimeName for ID2D1SvgElement {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1SvgPaint, ID2D1SvgPaint_Vtbl, 0xd59bab0a_68a2_455b_a5dc_9eb2854e2490);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1SvgPaint {
    type Target = ID2D1SvgAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1SvgPaint, windows_core::IUnknown, super::d2d1::ID2D1Resource, ID2D1SvgAttribute);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1SvgPaint {
    pub unsafe fn SetPaintType(&self, painttype: D2D1_SVG_PAINT_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPaintType)(windows_core::Interface::as_raw(self), painttype) }
    }
    pub unsafe fn GetPaintType(&self) -> D2D1_SVG_PAINT_TYPE {
        unsafe { (windows_core::Interface::vtable(self).GetPaintType)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub unsafe fn SetColor(&self, color: *const super::d2dbasetypes::D2D_COLOR_F) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetColor)(windows_core::Interface::as_raw(self), color) }
    }
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub unsafe fn GetColor(&self) -> super::d2dbasetypes::D2D_COLOR_F {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColor)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn SetId<P0>(&self, id: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetId)(windows_core::Interface::as_raw(self), id.param().abi()) }
    }
    pub unsafe fn GetId(&self, id: &mut [u16]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetId)(windows_core::Interface::as_raw(self), core::mem::transmute(id.as_ptr()), id.len().try_into().unwrap()) }
    }
    pub unsafe fn GetIdLength(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetIdLength)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1SvgPaint_Vtbl {
    pub base__: ID2D1SvgAttribute_Vtbl,
    pub SetPaintType: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_SVG_PAINT_TYPE) -> windows_core::HRESULT,
    pub GetPaintType: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_SVG_PAINT_TYPE,
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub SetColor: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::d2dbasetypes::D2D_COLOR_F) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype")))]
    SetColor: usize,
    #[cfg(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
    pub GetColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::d2dbasetypes::D2D_COLOR_F),
    #[cfg(not(all(feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype")))]
    GetColor: usize,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetIdLength: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
pub trait ID2D1SvgPaint_Impl: ID2D1SvgAttribute_Impl {
    fn SetPaintType(&self, painttype: D2D1_SVG_PAINT_TYPE) -> windows_core::Result<()>;
    fn GetPaintType(&self) -> D2D1_SVG_PAINT_TYPE;
    fn SetColor(&self, color: *const super::d2dbasetypes::D2D_COLOR_F) -> windows_core::Result<()>;
    fn GetColor(&self, color: *mut super::d2dbasetypes::D2D_COLOR_F);
    fn SetId(&self, id: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetId(&self, id: windows_core::PWSTR, idcount: u32) -> windows_core::Result<()>;
    fn GetIdLength(&self) -> u32;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
impl ID2D1SvgPaint_Vtbl {
    pub const fn new<Identity: ID2D1SvgPaint_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPaintType<Identity: ID2D1SvgPaint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, painttype: D2D1_SVG_PAINT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgPaint_Impl::SetPaintType(this, core::mem::transmute_copy(&painttype)).into()
            }
        }
        unsafe extern "system" fn GetPaintType<Identity: ID2D1SvgPaint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_SVG_PAINT_TYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgPaint_Impl::GetPaintType(this)
            }
        }
        unsafe extern "system" fn SetColor<Identity: ID2D1SvgPaint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *const super::d2dbasetypes::D2D_COLOR_F) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgPaint_Impl::SetColor(this, core::mem::transmute_copy(&color)).into()
            }
        }
        unsafe extern "system" fn GetColor<Identity: ID2D1SvgPaint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *mut super::d2dbasetypes::D2D_COLOR_F) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgPaint_Impl::GetColor(this, core::mem::transmute_copy(&color));
            }
        }
        unsafe extern "system" fn SetId<Identity: ID2D1SvgPaint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgPaint_Impl::SetId(this, core::mem::transmute(&id)).into()
            }
        }
        unsafe extern "system" fn GetId<Identity: ID2D1SvgPaint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: windows_core::PWSTR, idcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgPaint_Impl::GetId(this, core::mem::transmute_copy(&id), core::mem::transmute_copy(&idcount)).into()
            }
        }
        unsafe extern "system" fn GetIdLength<Identity: ID2D1SvgPaint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgPaint_Impl::GetIdLength(this)
            }
        }
        Self {
            base__: ID2D1SvgAttribute_Vtbl::new::<Identity, OFFSET>(),
            SetPaintType: SetPaintType::<Identity, OFFSET>,
            GetPaintType: GetPaintType::<Identity, OFFSET>,
            SetColor: SetColor::<Identity, OFFSET>,
            GetColor: GetColor::<Identity, OFFSET>,
            SetId: SetId::<Identity, OFFSET>,
            GetId: GetId::<Identity, OFFSET>,
            GetIdLength: GetIdLength::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1SvgPaint as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1SvgAttribute as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2dbasetypes", feature = "Win32_dxgitype"))]
impl windows_core::RuntimeName for ID2D1SvgPaint {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1SvgPathData, ID2D1SvgPathData_Vtbl, 0xc095e4f4_bb98_43d6_9745_4d1b84ec9888);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1SvgPathData {
    type Target = ID2D1SvgAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1SvgPathData, windows_core::IUnknown, super::d2d1::ID2D1Resource, ID2D1SvgAttribute);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1SvgPathData {
    pub unsafe fn RemoveSegmentDataAtEnd(&self, datacount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveSegmentDataAtEnd)(windows_core::Interface::as_raw(self), datacount) }
    }
    pub unsafe fn UpdateSegmentData(&self, data: &[f32], startindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateSegmentData)(windows_core::Interface::as_raw(self), core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap(), startindex) }
    }
    pub unsafe fn GetSegmentData(&self, data: &mut [f32], startindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSegmentData)(windows_core::Interface::as_raw(self), core::mem::transmute(data.as_ptr()), data.len().try_into().unwrap(), startindex) }
    }
    pub unsafe fn GetSegmentDataCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetSegmentDataCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn RemoveCommandsAtEnd(&self, commandscount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveCommandsAtEnd)(windows_core::Interface::as_raw(self), commandscount) }
    }
    pub unsafe fn UpdateCommands(&self, commands: &[D2D1_SVG_PATH_COMMAND], startindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateCommands)(windows_core::Interface::as_raw(self), core::mem::transmute(commands.as_ptr()), commands.len().try_into().unwrap(), startindex) }
    }
    pub unsafe fn GetCommands(&self, commands: &mut [D2D1_SVG_PATH_COMMAND], startindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCommands)(windows_core::Interface::as_raw(self), core::mem::transmute(commands.as_ptr()), commands.len().try_into().unwrap(), startindex) }
    }
    pub unsafe fn GetCommandsCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetCommandsCount)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_d2d1_1")]
    pub unsafe fn CreatePathGeometry(&self, fillmode: super::d2d1::D2D1_FILL_MODE) -> windows_core::Result<super::d2d1_1::ID2D1PathGeometry1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePathGeometry)(windows_core::Interface::as_raw(self), fillmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1SvgPathData_Vtbl {
    pub base__: ID2D1SvgAttribute_Vtbl,
    pub RemoveSegmentDataAtEnd: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UpdateSegmentData: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32, u32, u32) -> windows_core::HRESULT,
    pub GetSegmentData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32, u32) -> windows_core::HRESULT,
    pub GetSegmentDataCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub RemoveCommandsAtEnd: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UpdateCommands: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_SVG_PATH_COMMAND, u32, u32) -> windows_core::HRESULT,
    pub GetCommands: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_SVG_PATH_COMMAND, u32, u32) -> windows_core::HRESULT,
    pub GetCommandsCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_d2d1_1")]
    pub CreatePathGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, super::d2d1::D2D1_FILL_MODE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_d2d1_1"))]
    CreatePathGeometry: usize,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
pub trait ID2D1SvgPathData_Impl: ID2D1SvgAttribute_Impl {
    fn RemoveSegmentDataAtEnd(&self, datacount: u32) -> windows_core::Result<()>;
    fn UpdateSegmentData(&self, data: *const f32, datacount: u32, startindex: u32) -> windows_core::Result<()>;
    fn GetSegmentData(&self, data: *mut f32, datacount: u32, startindex: u32) -> windows_core::Result<()>;
    fn GetSegmentDataCount(&self) -> u32;
    fn RemoveCommandsAtEnd(&self, commandscount: u32) -> windows_core::Result<()>;
    fn UpdateCommands(&self, commands: *const D2D1_SVG_PATH_COMMAND, commandscount: u32, startindex: u32) -> windows_core::Result<()>;
    fn GetCommands(&self, commands: *mut D2D1_SVG_PATH_COMMAND, commandscount: u32, startindex: u32) -> windows_core::Result<()>;
    fn GetCommandsCount(&self) -> u32;
    fn CreatePathGeometry(&self, fillmode: super::d2d1::D2D1_FILL_MODE) -> windows_core::Result<super::d2d1_1::ID2D1PathGeometry1>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
impl ID2D1SvgPathData_Vtbl {
    pub const fn new<Identity: ID2D1SvgPathData_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RemoveSegmentDataAtEnd<Identity: ID2D1SvgPathData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datacount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgPathData_Impl::RemoveSegmentDataAtEnd(this, core::mem::transmute_copy(&datacount)).into()
            }
        }
        unsafe extern "system" fn UpdateSegmentData<Identity: ID2D1SvgPathData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *const f32, datacount: u32, startindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgPathData_Impl::UpdateSegmentData(this, core::mem::transmute_copy(&data), core::mem::transmute_copy(&datacount), core::mem::transmute_copy(&startindex)).into()
            }
        }
        unsafe extern "system" fn GetSegmentData<Identity: ID2D1SvgPathData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut f32, datacount: u32, startindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgPathData_Impl::GetSegmentData(this, core::mem::transmute_copy(&data), core::mem::transmute_copy(&datacount), core::mem::transmute_copy(&startindex)).into()
            }
        }
        unsafe extern "system" fn GetSegmentDataCount<Identity: ID2D1SvgPathData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgPathData_Impl::GetSegmentDataCount(this)
            }
        }
        unsafe extern "system" fn RemoveCommandsAtEnd<Identity: ID2D1SvgPathData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commandscount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgPathData_Impl::RemoveCommandsAtEnd(this, core::mem::transmute_copy(&commandscount)).into()
            }
        }
        unsafe extern "system" fn UpdateCommands<Identity: ID2D1SvgPathData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commands: *const D2D1_SVG_PATH_COMMAND, commandscount: u32, startindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgPathData_Impl::UpdateCommands(this, core::mem::transmute_copy(&commands), core::mem::transmute_copy(&commandscount), core::mem::transmute_copy(&startindex)).into()
            }
        }
        unsafe extern "system" fn GetCommands<Identity: ID2D1SvgPathData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commands: *mut D2D1_SVG_PATH_COMMAND, commandscount: u32, startindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgPathData_Impl::GetCommands(this, core::mem::transmute_copy(&commands), core::mem::transmute_copy(&commandscount), core::mem::transmute_copy(&startindex)).into()
            }
        }
        unsafe extern "system" fn GetCommandsCount<Identity: ID2D1SvgPathData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgPathData_Impl::GetCommandsCount(this)
            }
        }
        unsafe extern "system" fn CreatePathGeometry<Identity: ID2D1SvgPathData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fillmode: super::d2d1::D2D1_FILL_MODE, pathgeometry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1SvgPathData_Impl::CreatePathGeometry(this, core::mem::transmute_copy(&fillmode)) {
                    Ok(ok__) => {
                        pathgeometry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ID2D1SvgAttribute_Vtbl::new::<Identity, OFFSET>(),
            RemoveSegmentDataAtEnd: RemoveSegmentDataAtEnd::<Identity, OFFSET>,
            UpdateSegmentData: UpdateSegmentData::<Identity, OFFSET>,
            GetSegmentData: GetSegmentData::<Identity, OFFSET>,
            GetSegmentDataCount: GetSegmentDataCount::<Identity, OFFSET>,
            RemoveCommandsAtEnd: RemoveCommandsAtEnd::<Identity, OFFSET>,
            UpdateCommands: UpdateCommands::<Identity, OFFSET>,
            GetCommands: GetCommands::<Identity, OFFSET>,
            GetCommandsCount: GetCommandsCount::<Identity, OFFSET>,
            CreatePathGeometry: CreatePathGeometry::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1SvgPathData as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1SvgAttribute as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
impl windows_core::RuntimeName for ID2D1SvgPathData {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1SvgPointCollection, ID2D1SvgPointCollection_Vtbl, 0x9dbe4c0d_3572_4dd9_9825_5530813bb712);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1SvgPointCollection {
    type Target = ID2D1SvgAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1SvgPointCollection, windows_core::IUnknown, super::d2d1::ID2D1Resource, ID2D1SvgAttribute);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1SvgPointCollection {
    pub unsafe fn RemovePointsAtEnd(&self, pointscount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemovePointsAtEnd)(windows_core::Interface::as_raw(self), pointscount) }
    }
    pub unsafe fn UpdatePoints(&self, points: &[windows_numerics::Vector2], startindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdatePoints)(windows_core::Interface::as_raw(self), core::mem::transmute(points.as_ptr()), points.len().try_into().unwrap(), startindex) }
    }
    pub unsafe fn GetPoints(&self, points: &mut [windows_numerics::Vector2], startindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPoints)(windows_core::Interface::as_raw(self), core::mem::transmute(points.as_ptr()), points.len().try_into().unwrap(), startindex) }
    }
    pub unsafe fn GetPointsCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetPointsCount)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1SvgPointCollection_Vtbl {
    pub base__: ID2D1SvgAttribute_Vtbl,
    pub RemovePointsAtEnd: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UpdatePoints: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Vector2, u32, u32) -> windows_core::HRESULT,
    pub GetPoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Vector2, u32, u32) -> windows_core::HRESULT,
    pub GetPointsCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
#[cfg(feature = "Win32_d2d1")]
pub trait ID2D1SvgPointCollection_Impl: ID2D1SvgAttribute_Impl {
    fn RemovePointsAtEnd(&self, pointscount: u32) -> windows_core::Result<()>;
    fn UpdatePoints(&self, points: *const windows_numerics::Vector2, pointscount: u32, startindex: u32) -> windows_core::Result<()>;
    fn GetPoints(&self, points: *mut windows_numerics::Vector2, pointscount: u32, startindex: u32) -> windows_core::Result<()>;
    fn GetPointsCount(&self) -> u32;
}
#[cfg(feature = "Win32_d2d1")]
impl ID2D1SvgPointCollection_Vtbl {
    pub const fn new<Identity: ID2D1SvgPointCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RemovePointsAtEnd<Identity: ID2D1SvgPointCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointscount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgPointCollection_Impl::RemovePointsAtEnd(this, core::mem::transmute_copy(&pointscount)).into()
            }
        }
        unsafe extern "system" fn UpdatePoints<Identity: ID2D1SvgPointCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, points: *const windows_numerics::Vector2, pointscount: u32, startindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgPointCollection_Impl::UpdatePoints(this, core::mem::transmute_copy(&points), core::mem::transmute_copy(&pointscount), core::mem::transmute_copy(&startindex)).into()
            }
        }
        unsafe extern "system" fn GetPoints<Identity: ID2D1SvgPointCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, points: *mut windows_numerics::Vector2, pointscount: u32, startindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgPointCollection_Impl::GetPoints(this, core::mem::transmute_copy(&points), core::mem::transmute_copy(&pointscount), core::mem::transmute_copy(&startindex)).into()
            }
        }
        unsafe extern "system" fn GetPointsCount<Identity: ID2D1SvgPointCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgPointCollection_Impl::GetPointsCount(this)
            }
        }
        Self {
            base__: ID2D1SvgAttribute_Vtbl::new::<Identity, OFFSET>(),
            RemovePointsAtEnd: RemovePointsAtEnd::<Identity, OFFSET>,
            UpdatePoints: UpdatePoints::<Identity, OFFSET>,
            GetPoints: GetPoints::<Identity, OFFSET>,
            GetPointsCount: GetPointsCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1SvgPointCollection as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1SvgAttribute as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d2d1")]
impl windows_core::RuntimeName for ID2D1SvgPointCollection {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1SvgStrokeDashArray, ID2D1SvgStrokeDashArray_Vtbl, 0xf1c0ca52_92a3_4f00_b4ce_f35691efd9d9);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1SvgStrokeDashArray {
    type Target = ID2D1SvgAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1SvgStrokeDashArray, windows_core::IUnknown, super::d2d1::ID2D1Resource, ID2D1SvgAttribute);
#[cfg(feature = "Win32_d2d1")]
impl ID2D1SvgStrokeDashArray {
    pub unsafe fn RemoveDashesAtEnd(&self, dashescount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveDashesAtEnd)(windows_core::Interface::as_raw(self), dashescount) }
    }
    pub unsafe fn UpdateDashes(&self, dashes: &[f32], startindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateDashes)(windows_core::Interface::as_raw(self), core::mem::transmute(dashes.as_ptr()), dashes.len().try_into().unwrap(), startindex) }
    }
    pub unsafe fn UpdateDashes2(&self, dashes: &[D2D1_SVG_LENGTH], startindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateDashes2)(windows_core::Interface::as_raw(self), core::mem::transmute(dashes.as_ptr()), dashes.len().try_into().unwrap(), startindex) }
    }
    pub unsafe fn GetDashes(&self, dashes: &mut [f32], startindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDashes)(windows_core::Interface::as_raw(self), core::mem::transmute(dashes.as_ptr()), dashes.len().try_into().unwrap(), startindex) }
    }
    pub unsafe fn GetDashes2(&self, dashes: &mut [D2D1_SVG_LENGTH], startindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDashes2)(windows_core::Interface::as_raw(self), core::mem::transmute(dashes.as_ptr()), dashes.len().try_into().unwrap(), startindex) }
    }
    pub unsafe fn GetDashesCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetDashesCount)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1SvgStrokeDashArray_Vtbl {
    pub base__: ID2D1SvgAttribute_Vtbl,
    pub RemoveDashesAtEnd: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub UpdateDashes: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32, u32, u32) -> windows_core::HRESULT,
    pub UpdateDashes2: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_SVG_LENGTH, u32, u32) -> windows_core::HRESULT,
    pub GetDashes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32, u32) -> windows_core::HRESULT,
    pub GetDashes2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_SVG_LENGTH, u32, u32) -> windows_core::HRESULT,
    pub GetDashesCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
#[cfg(feature = "Win32_d2d1")]
pub trait ID2D1SvgStrokeDashArray_Impl: ID2D1SvgAttribute_Impl {
    fn RemoveDashesAtEnd(&self, dashescount: u32) -> windows_core::Result<()>;
    fn UpdateDashes(&self, dashes: *const f32, dashescount: u32, startindex: u32) -> windows_core::Result<()>;
    fn UpdateDashes2(&self, dashes: *const D2D1_SVG_LENGTH, dashescount: u32, startindex: u32) -> windows_core::Result<()>;
    fn GetDashes(&self, dashes: *mut f32, dashescount: u32, startindex: u32) -> windows_core::Result<()>;
    fn GetDashes2(&self, dashes: *mut D2D1_SVG_LENGTH, dashescount: u32, startindex: u32) -> windows_core::Result<()>;
    fn GetDashesCount(&self) -> u32;
}
#[cfg(feature = "Win32_d2d1")]
impl ID2D1SvgStrokeDashArray_Vtbl {
    pub const fn new<Identity: ID2D1SvgStrokeDashArray_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RemoveDashesAtEnd<Identity: ID2D1SvgStrokeDashArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dashescount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgStrokeDashArray_Impl::RemoveDashesAtEnd(this, core::mem::transmute_copy(&dashescount)).into()
            }
        }
        unsafe extern "system" fn UpdateDashes<Identity: ID2D1SvgStrokeDashArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dashes: *const f32, dashescount: u32, startindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgStrokeDashArray_Impl::UpdateDashes(this, core::mem::transmute_copy(&dashes), core::mem::transmute_copy(&dashescount), core::mem::transmute_copy(&startindex)).into()
            }
        }
        unsafe extern "system" fn UpdateDashes2<Identity: ID2D1SvgStrokeDashArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dashes: *const D2D1_SVG_LENGTH, dashescount: u32, startindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgStrokeDashArray_Impl::UpdateDashes2(this, core::mem::transmute_copy(&dashes), core::mem::transmute_copy(&dashescount), core::mem::transmute_copy(&startindex)).into()
            }
        }
        unsafe extern "system" fn GetDashes<Identity: ID2D1SvgStrokeDashArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dashes: *mut f32, dashescount: u32, startindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgStrokeDashArray_Impl::GetDashes(this, core::mem::transmute_copy(&dashes), core::mem::transmute_copy(&dashescount), core::mem::transmute_copy(&startindex)).into()
            }
        }
        unsafe extern "system" fn GetDashes2<Identity: ID2D1SvgStrokeDashArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dashes: *mut D2D1_SVG_LENGTH, dashescount: u32, startindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgStrokeDashArray_Impl::GetDashes2(this, core::mem::transmute_copy(&dashes), core::mem::transmute_copy(&dashescount), core::mem::transmute_copy(&startindex)).into()
            }
        }
        unsafe extern "system" fn GetDashesCount<Identity: ID2D1SvgStrokeDashArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SvgStrokeDashArray_Impl::GetDashesCount(this)
            }
        }
        Self {
            base__: ID2D1SvgAttribute_Vtbl::new::<Identity, OFFSET>(),
            RemoveDashesAtEnd: RemoveDashesAtEnd::<Identity, OFFSET>,
            UpdateDashes: UpdateDashes::<Identity, OFFSET>,
            UpdateDashes2: UpdateDashes2::<Identity, OFFSET>,
            GetDashes: GetDashes::<Identity, OFFSET>,
            GetDashes2: GetDashes2::<Identity, OFFSET>,
            GetDashesCount: GetDashesCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1SvgStrokeDashArray as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<ID2D1SvgAttribute as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d2d1")]
impl windows_core::RuntimeName for ID2D1SvgStrokeDashArray {}
