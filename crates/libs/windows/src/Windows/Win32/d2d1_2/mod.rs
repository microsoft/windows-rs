#[inline]
pub unsafe fn D2D1ComputeMaximumScaleFactor(matrix: *const windows_numerics::Matrix3x2) -> f32 {
    windows_core::link!("d2d1.dll" "system" fn D2D1ComputeMaximumScaleFactor(matrix : *const windows_numerics::Matrix3x2) -> f32);
    unsafe { D2D1ComputeMaximumScaleFactor(matrix) }
}
pub type D2D1_RENDERING_PRIORITY = i32;
pub const D2D1_RENDERING_PRIORITY_FORCE_DWORD: D2D1_RENDERING_PRIORITY = -1;
pub const D2D1_RENDERING_PRIORITY_LOW: D2D1_RENDERING_PRIORITY = 1;
pub const D2D1_RENDERING_PRIORITY_NORMAL: D2D1_RENDERING_PRIORITY = 0;
#[cfg(feature = "Win32_d2d1_1")]
windows_core::imp::define_interface!(ID2D1CommandSink1, ID2D1CommandSink1_Vtbl, 0x9eb767fd_4269_4467_b8c2_eb30cb305743);
#[cfg(feature = "Win32_d2d1_1")]
impl core::ops::Deref for ID2D1CommandSink1 {
    type Target = super::d2d1_1::ID2D1CommandSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1_1")]
windows_core::imp::interface_hierarchy!(ID2D1CommandSink1, windows_core::IUnknown, super::d2d1_1::ID2D1CommandSink);
#[cfg(feature = "Win32_d2d1_1")]
impl ID2D1CommandSink1 {
    pub unsafe fn SetPrimitiveBlend1(&self, primitiveblend: super::d2d1_1::D2D1_PRIMITIVE_BLEND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPrimitiveBlend1)(windows_core::Interface::as_raw(self), primitiveblend) }
    }
}
#[cfg(feature = "Win32_d2d1_1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1CommandSink1_Vtbl {
    pub base__: super::d2d1_1::ID2D1CommandSink_Vtbl,
    pub SetPrimitiveBlend1: unsafe extern "system" fn(*mut core::ffi::c_void, super::d2d1_1::D2D1_PRIMITIVE_BLEND) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
pub trait ID2D1CommandSink1_Impl: super::d2d1_1::ID2D1CommandSink_Impl {
    fn SetPrimitiveBlend1(&self, primitiveblend: super::d2d1_1::D2D1_PRIMITIVE_BLEND) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
impl ID2D1CommandSink1_Vtbl {
    pub const fn new<Identity: ID2D1CommandSink1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPrimitiveBlend1<Identity: ID2D1CommandSink1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, primitiveblend: super::d2d1_1::D2D1_PRIMITIVE_BLEND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1CommandSink1_Impl::SetPrimitiveBlend1(this, core::mem::transmute_copy(&primitiveblend)).into()
            }
        }
        Self { base__: super::d2d1_1::ID2D1CommandSink_Vtbl::new::<Identity, OFFSET>(), SetPrimitiveBlend1: SetPrimitiveBlend1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1CommandSink1 as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1CommandSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgitype"))]
impl windows_core::RuntimeName for ID2D1CommandSink1 {}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
windows_core::imp::define_interface!(ID2D1Device1, ID2D1Device1_Vtbl, 0xd21768e1_23a4_4823_a14b_7c3eba85d658);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
impl core::ops::Deref for ID2D1Device1 {
    type Target = super::d2d1_1::ID2D1Device;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
windows_core::imp::interface_hierarchy!(ID2D1Device1, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1_1::ID2D1Device);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
impl ID2D1Device1 {
    pub unsafe fn GetRenderingPriority(&self) -> D2D1_RENDERING_PRIORITY {
        unsafe { (windows_core::Interface::vtable(self).GetRenderingPriority)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetRenderingPriority(&self, renderingpriority: D2D1_RENDERING_PRIORITY) {
        unsafe {
            (windows_core::Interface::vtable(self).SetRenderingPriority)(windows_core::Interface::as_raw(self), renderingpriority);
        }
    }
    pub unsafe fn CreateDeviceContext(&self, options: super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDeviceContext)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Device1_Vtbl {
    pub base__: super::d2d1_1::ID2D1Device_Vtbl,
    pub GetRenderingPriority: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_RENDERING_PRIORITY,
    pub SetRenderingPriority: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_RENDERING_PRIORITY),
    pub CreateDeviceContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_wincodec"))]
pub trait ID2D1Device1_Impl: super::d2d1_1::ID2D1Device_Impl {
    fn GetRenderingPriority(&self) -> D2D1_RENDERING_PRIORITY;
    fn SetRenderingPriority(&self, renderingpriority: D2D1_RENDERING_PRIORITY);
    fn CreateDeviceContext(&self, options: super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext1>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_wincodec"))]
impl ID2D1Device1_Vtbl {
    pub const fn new<Identity: ID2D1Device1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRenderingPriority<Identity: ID2D1Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> D2D1_RENDERING_PRIORITY {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Device1_Impl::GetRenderingPriority(this)
            }
        }
        unsafe extern "system" fn SetRenderingPriority<Identity: ID2D1Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, renderingpriority: D2D1_RENDERING_PRIORITY) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Device1_Impl::SetRenderingPriority(this, core::mem::transmute_copy(&renderingpriority));
            }
        }
        unsafe extern "system" fn CreateDeviceContext<Identity: ID2D1Device1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: super::d2d1_1::D2D1_DEVICE_CONTEXT_OPTIONS, devicecontext1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Device1_Impl::CreateDeviceContext(this, core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        devicecontext1.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::d2d1_1::ID2D1Device_Vtbl::new::<Identity, OFFSET>(),
            GetRenderingPriority: GetRenderingPriority::<Identity, OFFSET>,
            SetRenderingPriority: SetRenderingPriority::<Identity, OFFSET>,
            CreateDeviceContext: CreateDeviceContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Device1 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1Device as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_wincodec"))]
impl windows_core::RuntimeName for ID2D1Device1 {}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
windows_core::imp::define_interface!(ID2D1DeviceContext1, ID2D1DeviceContext1_Vtbl, 0xd37f57e4_6908_459f_a199_e72f24f79987);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
impl core::ops::Deref for ID2D1DeviceContext1 {
    type Target = super::d2d1_1::ID2D1DeviceContext;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
windows_core::imp::interface_hierarchy!(ID2D1DeviceContext1, windows_core::IUnknown, super::d2d1::ID2D1Resource, super::d2d1::ID2D1RenderTarget, super::d2d1_1::ID2D1DeviceContext);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
impl ID2D1DeviceContext1 {
    pub unsafe fn CreateFilledGeometryRealization<P0>(&self, geometry: P0, flatteningtolerance: f32) -> windows_core::Result<ID2D1GeometryRealization>
    where
        P0: windows_core::Param<super::d2d1::ID2D1Geometry>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFilledGeometryRealization)(windows_core::Interface::as_raw(self), geometry.param().abi(), flatteningtolerance, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateStrokedGeometryRealization<P0, P3>(&self, geometry: P0, flatteningtolerance: f32, strokewidth: f32, strokestyle: P3) -> windows_core::Result<ID2D1GeometryRealization>
    where
        P0: windows_core::Param<super::d2d1::ID2D1Geometry>,
        P3: windows_core::Param<super::d2d1::ID2D1StrokeStyle>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStrokedGeometryRealization)(windows_core::Interface::as_raw(self), geometry.param().abi(), flatteningtolerance, strokewidth, strokestyle.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DrawGeometryRealization<P0, P1>(&self, geometryrealization: P0, brush: P1)
    where
        P0: windows_core::Param<ID2D1GeometryRealization>,
        P1: windows_core::Param<super::d2d1::ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawGeometryRealization)(windows_core::Interface::as_raw(self), geometryrealization.param().abi(), brush.param().abi());
        }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1DeviceContext1_Vtbl {
    pub base__: super::d2d1_1::ID2D1DeviceContext_Vtbl,
    pub CreateFilledGeometryRealization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateStrokedGeometryRealization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f32, f32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DrawGeometryRealization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec"))]
pub trait ID2D1DeviceContext1_Impl: super::d2d1_1::ID2D1DeviceContext_Impl {
    fn CreateFilledGeometryRealization(&self, geometry: windows_core::Ref<super::d2d1::ID2D1Geometry>, flatteningtolerance: f32) -> windows_core::Result<ID2D1GeometryRealization>;
    fn CreateStrokedGeometryRealization(&self, geometry: windows_core::Ref<super::d2d1::ID2D1Geometry>, flatteningtolerance: f32, strokewidth: f32, strokestyle: windows_core::Ref<super::d2d1::ID2D1StrokeStyle>) -> windows_core::Result<ID2D1GeometryRealization>;
    fn DrawGeometryRealization(&self, geometryrealization: windows_core::Ref<ID2D1GeometryRealization>, brush: windows_core::Ref<super::d2d1::ID2D1Brush>);
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec"))]
impl ID2D1DeviceContext1_Vtbl {
    pub const fn new<Identity: ID2D1DeviceContext1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateFilledGeometryRealization<Identity: ID2D1DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut core::ffi::c_void, flatteningtolerance: f32, geometryrealization: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext1_Impl::CreateFilledGeometryRealization(this, core::mem::transmute_copy(&geometry), core::mem::transmute_copy(&flatteningtolerance)) {
                    Ok(ok__) => {
                        geometryrealization.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateStrokedGeometryRealization<Identity: ID2D1DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometry: *mut core::ffi::c_void, flatteningtolerance: f32, strokewidth: f32, strokestyle: *mut core::ffi::c_void, geometryrealization: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1DeviceContext1_Impl::CreateStrokedGeometryRealization(this, core::mem::transmute_copy(&geometry), core::mem::transmute_copy(&flatteningtolerance), core::mem::transmute_copy(&strokewidth), core::mem::transmute_copy(&strokestyle)) {
                    Ok(ok__) => {
                        geometryrealization.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DrawGeometryRealization<Identity: ID2D1DeviceContext1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, geometryrealization: *mut core::ffi::c_void, brush: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1DeviceContext1_Impl::DrawGeometryRealization(this, core::mem::transmute_copy(&geometryrealization), core::mem::transmute_copy(&brush));
            }
        }
        Self {
            base__: super::d2d1_1::ID2D1DeviceContext_Vtbl::new::<Identity, OFFSET>(),
            CreateFilledGeometryRealization: CreateFilledGeometryRealization::<Identity, OFFSET>,
            CreateStrokedGeometryRealization: CreateStrokedGeometryRealization::<Identity, OFFSET>,
            DrawGeometryRealization: DrawGeometryRealization::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1DeviceContext1 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1RenderTarget as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1DeviceContext as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_d2dbasetypes", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_wincodec"))]
impl windows_core::RuntimeName for ID2D1DeviceContext1 {}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
windows_core::imp::define_interface!(ID2D1Factory2, ID2D1Factory2_Vtbl, 0x94f81a73_9212_4376_9c58_b16a3a0d3992);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
impl core::ops::Deref for ID2D1Factory2 {
    type Target = super::d2d1_1::ID2D1Factory1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
windows_core::imp::interface_hierarchy!(ID2D1Factory2, windows_core::IUnknown, super::d2d1::ID2D1Factory, super::d2d1_1::ID2D1Factory1);
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
impl ID2D1Factory2 {
    #[cfg(feature = "Win32_dxgi")]
    pub unsafe fn CreateDevice<P0>(&self, dxgidevice: P0) -> windows_core::Result<ID2D1Device1>
    where
        P0: windows_core::Param<super::dxgi::IDXGIDevice>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), dxgidevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Factory2_Vtbl {
    pub base__: super::d2d1_1::ID2D1Factory1_Vtbl,
    #[cfg(feature = "Win32_dxgi")]
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgi"))]
    CreateDevice: usize,
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_objidlbase", feature = "Win32_wincodec", feature = "Win32_windef"))]
pub trait ID2D1Factory2_Impl: super::d2d1_1::ID2D1Factory1_Impl {
    fn CreateDevice(&self, dxgidevice: windows_core::Ref<super::dxgi::IDXGIDevice>) -> windows_core::Result<ID2D1Device1>;
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_objidlbase", feature = "Win32_wincodec", feature = "Win32_windef"))]
impl ID2D1Factory2_Vtbl {
    pub const fn new<Identity: ID2D1Factory2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDevice<Identity: ID2D1Factory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxgidevice: *mut core::ffi::c_void, d2ddevice1: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory2_Impl::CreateDevice(this, core::mem::transmute_copy(&dxgidevice)) {
                    Ok(ok__) => {
                        d2ddevice1.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::d2d1_1::ID2D1Factory1_Vtbl::new::<Identity, OFFSET>(), CreateDevice: CreateDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Factory2 as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Factory as windows_core::Interface>::IID || iid == &<super::d2d1_1::ID2D1Factory1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d2d1", feature = "Win32_d2d1_1", feature = "Win32_dcommon", feature = "Win32_dwrite", feature = "Win32_dxgi", feature = "Win32_dxgiformat", feature = "Win32_objidlbase", feature = "Win32_wincodec", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID2D1Factory2 {}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::define_interface!(ID2D1GeometryRealization, ID2D1GeometryRealization_Vtbl, 0xa16907d7_bc02_4801_99e8_8cf7f485f774);
#[cfg(feature = "Win32_d2d1")]
impl core::ops::Deref for ID2D1GeometryRealization {
    type Target = super::d2d1::ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_d2d1")]
windows_core::imp::interface_hierarchy!(ID2D1GeometryRealization, windows_core::IUnknown, super::d2d1::ID2D1Resource);
#[cfg(feature = "Win32_d2d1")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1GeometryRealization_Vtbl {
    pub base__: super::d2d1::ID2D1Resource_Vtbl,
}
#[cfg(feature = "Win32_d2d1")]
pub trait ID2D1GeometryRealization_Impl: super::d2d1::ID2D1Resource_Impl {}
#[cfg(feature = "Win32_d2d1")]
impl ID2D1GeometryRealization_Vtbl {
    pub const fn new<Identity: ID2D1GeometryRealization_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::d2d1::ID2D1Resource_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1GeometryRealization as windows_core::Interface>::IID || iid == &<super::d2d1::ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_d2d1")]
impl windows_core::RuntimeName for ID2D1GeometryRealization {}
