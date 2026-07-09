pub type D3D11_CHECK_MULTISAMPLE_QUALITY_LEVELS_FLAG = i32;
pub const D3D11_CHECK_MULTISAMPLE_QUALITY_LEVELS_TILED_RESOURCE: D3D11_CHECK_MULTISAMPLE_QUALITY_LEVELS_FLAG = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_PACKED_MIP_DESC {
    pub NumStandardMips: u8,
    pub NumPackedMips: u8,
    pub NumTilesForPackedMips: u32,
    pub StartTileIndexInOverallResource: u32,
}
pub const D3D11_PACKED_TILE: u32 = 4294967295;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_SUBRESOURCE_TILING {
    pub WidthInTiles: u32,
    pub HeightInTiles: u16,
    pub DepthInTiles: u16,
    pub StartTileIndexInOverallResource: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TILED_RESOURCE_COORDINATE {
    pub X: u32,
    pub Y: u32,
    pub Z: u32,
    pub Subresource: u32,
}
pub type D3D11_TILE_COPY_FLAG = i32;
pub const D3D11_TILE_COPY_LINEAR_BUFFER_TO_SWIZZLED_TILED_RESOURCE: D3D11_TILE_COPY_FLAG = 2;
pub const D3D11_TILE_COPY_NO_OVERWRITE: D3D11_TILE_COPY_FLAG = 1;
pub const D3D11_TILE_COPY_SWIZZLED_TILED_RESOURCE_TO_LINEAR_BUFFER: D3D11_TILE_COPY_FLAG = 4;
pub type D3D11_TILE_MAPPING_FLAG = i32;
pub const D3D11_TILE_MAPPING_NO_OVERWRITE: D3D11_TILE_MAPPING_FLAG = 1;
pub type D3D11_TILE_RANGE_FLAG = i32;
pub const D3D11_TILE_RANGE_NULL: D3D11_TILE_RANGE_FLAG = 1;
pub const D3D11_TILE_RANGE_REUSE_SINGLE_TILE: D3D11_TILE_RANGE_FLAG = 4;
pub const D3D11_TILE_RANGE_SKIP: D3D11_TILE_RANGE_FLAG = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TILE_REGION_SIZE {
    pub NumTiles: u32,
    pub bUseBox: windows_core::BOOL,
    pub Width: u32,
    pub Height: u16,
    pub Depth: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_TILE_SHAPE {
    pub WidthInTexels: u32,
    pub HeightInTexels: u32,
    pub DepthInTexels: u32,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
windows_core::imp::define_interface!(ID3D11Device2, ID3D11Device2_Vtbl, 0x9d06dffa_d1e5_4d07_83a8_1bb123f2f841);
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
impl core::ops::Deref for ID3D11Device2 {
    type Target = super::d3d11_1::ID3D11Device1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
windows_core::imp::interface_hierarchy!(ID3D11Device2, windows_core::IUnknown, super::d3d11::ID3D11Device, super::d3d11_1::ID3D11Device1);
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
impl ID3D11Device2 {
    pub unsafe fn GetImmediateContext2(&self) -> windows_core::Result<ID3D11DeviceContext2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImmediateContext2)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::Type::from_abi(result__)
        }
    }
    pub unsafe fn CreateDeferredContext2(&self, contextflags: u32, ppdeferredcontext: Option<*mut Option<ID3D11DeviceContext2>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateDeferredContext2)(windows_core::Interface::as_raw(self), contextflags, ppdeferredcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetResourceTiling<P0>(&self, ptiledresource: P0, pnumtilesforentireresource: Option<*mut u32>, ppackedmipdesc: Option<*mut D3D11_PACKED_MIP_DESC>, pstandardtileshapefornonpackedmips: Option<*mut D3D11_TILE_SHAPE>, pnumsubresourcetilings: Option<*mut u32>, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D11_SUBRESOURCE_TILING)
    where
        P0: windows_core::Param<super::d3d11::ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GetResourceTiling)(windows_core::Interface::as_raw(self), ptiledresource.param().abi(), pnumtilesforentireresource.unwrap_or(core::mem::zeroed()) as _, ppackedmipdesc.unwrap_or(core::mem::zeroed()) as _, pstandardtileshapefornonpackedmips.unwrap_or(core::mem::zeroed()) as _, pnumsubresourcetilings.unwrap_or(core::mem::zeroed()) as _, firstsubresourcetilingtoget, psubresourcetilingsfornonpackedmips as _);
        }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn CheckMultisampleQualityLevels1(&self, format: super::dxgiformat::DXGI_FORMAT, samplecount: u32, flags: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckMultisampleQualityLevels1)(windows_core::Interface::as_raw(self), format, samplecount, flags, &mut result__).map(|| result__)
        }
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Device2_Vtbl {
    pub base__: super::d3d11_1::ID3D11Device1_Vtbl,
    pub GetImmediateContext2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub CreateDeferredContext2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResourceTiling: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *mut D3D11_PACKED_MIP_DESC, *mut D3D11_TILE_SHAPE, *mut u32, u32, *mut D3D11_SUBRESOURCE_TILING),
    #[cfg(feature = "Win32_dxgiformat")]
    pub CheckMultisampleQualityLevels1: unsafe extern "system" fn(*mut core::ffi::c_void, super::dxgiformat::DXGI_FORMAT, u32, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    CheckMultisampleQualityLevels1: usize,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3dcommon", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
pub trait ID3D11Device2_Impl: super::d3d11_1::ID3D11Device1_Impl {
    fn GetImmediateContext2(&self, ppimmediatecontext: windows_core::OutRef<ID3D11DeviceContext2>);
    fn CreateDeferredContext2(&self, contextflags: u32, ppdeferredcontext: windows_core::OutRef<ID3D11DeviceContext2>) -> windows_core::Result<()>;
    fn GetResourceTiling(&self, ptiledresource: windows_core::Ref<super::d3d11::ID3D11Resource>, pnumtilesforentireresource: *mut u32, ppackedmipdesc: *mut D3D11_PACKED_MIP_DESC, pstandardtileshapefornonpackedmips: *mut D3D11_TILE_SHAPE, pnumsubresourcetilings: *mut u32, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D11_SUBRESOURCE_TILING);
    fn CheckMultisampleQualityLevels1(&self, format: super::dxgiformat::DXGI_FORMAT, samplecount: u32, flags: u32) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3dcommon", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
impl ID3D11Device2_Vtbl {
    pub const fn new<Identity: ID3D11Device2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetImmediateContext2<Identity: ID3D11Device2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppimmediatecontext: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device2_Impl::GetImmediateContext2(this, core::mem::transmute_copy(&ppimmediatecontext));
            }
        }
        unsafe extern "system" fn CreateDeferredContext2<Identity: ID3D11Device2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contextflags: u32, ppdeferredcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device2_Impl::CreateDeferredContext2(this, core::mem::transmute_copy(&contextflags), core::mem::transmute_copy(&ppdeferredcontext)).into()
            }
        }
        unsafe extern "system" fn GetResourceTiling<Identity: ID3D11Device2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptiledresource: *mut core::ffi::c_void, pnumtilesforentireresource: *mut u32, ppackedmipdesc: *mut D3D11_PACKED_MIP_DESC, pstandardtileshapefornonpackedmips: *mut D3D11_TILE_SHAPE, pnumsubresourcetilings: *mut u32, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D11_SUBRESOURCE_TILING) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device2_Impl::GetResourceTiling(this, core::mem::transmute_copy(&ptiledresource), core::mem::transmute_copy(&pnumtilesforentireresource), core::mem::transmute_copy(&ppackedmipdesc), core::mem::transmute_copy(&pstandardtileshapefornonpackedmips), core::mem::transmute_copy(&pnumsubresourcetilings), core::mem::transmute_copy(&firstsubresourcetilingtoget), core::mem::transmute_copy(&psubresourcetilingsfornonpackedmips));
            }
        }
        unsafe extern "system" fn CheckMultisampleQualityLevels1<Identity: ID3D11Device2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: super::dxgiformat::DXGI_FORMAT, samplecount: u32, flags: u32, pnumqualitylevels: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11Device2_Impl::CheckMultisampleQualityLevels1(this, core::mem::transmute_copy(&format), core::mem::transmute_copy(&samplecount), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pnumqualitylevels.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::d3d11_1::ID3D11Device1_Vtbl::new::<Identity, OFFSET>(),
            GetImmediateContext2: GetImmediateContext2::<Identity, OFFSET>,
            CreateDeferredContext2: CreateDeferredContext2::<Identity, OFFSET>,
            GetResourceTiling: GetResourceTiling::<Identity, OFFSET>,
            CheckMultisampleQualityLevels1: CheckMultisampleQualityLevels1::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Device2 as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11Device as windows_core::Interface>::IID || iid == &<super::d3d11_1::ID3D11Device1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3dcommon", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for ID3D11Device2 {}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
windows_core::imp::define_interface!(ID3D11DeviceContext2, ID3D11DeviceContext2_Vtbl, 0x420d5b32_b90c_4da4_bef0_359f6a24a83a);
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
impl core::ops::Deref for ID3D11DeviceContext2 {
    type Target = super::d3d11_1::ID3D11DeviceContext1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
windows_core::imp::interface_hierarchy!(ID3D11DeviceContext2, windows_core::IUnknown, super::d3d11::ID3D11DeviceChild, super::d3d11::ID3D11DeviceContext, super::d3d11_1::ID3D11DeviceContext1);
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
impl ID3D11DeviceContext2 {
    pub unsafe fn UpdateTileMappings<P0, P4>(&self, ptiledresource: P0, numtiledresourceregions: u32, ptiledresourceregionstartcoordinates: Option<*const D3D11_TILED_RESOURCE_COORDINATE>, ptiledresourceregionsizes: Option<*const D3D11_TILE_REGION_SIZE>, ptilepool: P4, numranges: u32, prangeflags: Option<*const u32>, ptilepoolstartoffsets: Option<*const u32>, prangetilecounts: Option<*const u32>, flags: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d3d11::ID3D11Resource>,
        P4: windows_core::Param<super::d3d11::ID3D11Buffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateTileMappings)(windows_core::Interface::as_raw(self), ptiledresource.param().abi(), numtiledresourceregions, ptiledresourceregionstartcoordinates.unwrap_or(core::mem::zeroed()) as _, ptiledresourceregionsizes.unwrap_or(core::mem::zeroed()) as _, ptilepool.param().abi(), numranges, prangeflags.unwrap_or(core::mem::zeroed()) as _, ptilepoolstartoffsets.unwrap_or(core::mem::zeroed()) as _, prangetilecounts.unwrap_or(core::mem::zeroed()) as _, flags) }
    }
    pub unsafe fn CopyTileMappings<P0, P2>(&self, pdesttiledresource: P0, pdestregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, psourcetiledresource: P2, psourceregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, flags: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d3d11::ID3D11Resource>,
        P2: windows_core::Param<super::d3d11::ID3D11Resource>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyTileMappings)(windows_core::Interface::as_raw(self), pdesttiledresource.param().abi(), pdestregionstartcoordinate, psourcetiledresource.param().abi(), psourceregionstartcoordinate, ptileregionsize, flags) }
    }
    pub unsafe fn CopyTiles<P0, P3>(&self, ptiledresource: P0, ptileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, pbuffer: P3, bufferstartoffsetinbytes: u64, flags: u32)
    where
        P0: windows_core::Param<super::d3d11::ID3D11Resource>,
        P3: windows_core::Param<super::d3d11::ID3D11Buffer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CopyTiles)(windows_core::Interface::as_raw(self), ptiledresource.param().abi(), ptileregionstartcoordinate, ptileregionsize, pbuffer.param().abi(), bufferstartoffsetinbytes, flags);
        }
    }
    pub unsafe fn UpdateTiles<P0>(&self, pdesttiledresource: P0, pdesttileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, pdesttileregionsize: *const D3D11_TILE_REGION_SIZE, psourcetiledata: *const core::ffi::c_void, flags: u32)
    where
        P0: windows_core::Param<super::d3d11::ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).UpdateTiles)(windows_core::Interface::as_raw(self), pdesttiledresource.param().abi(), pdesttileregionstartcoordinate, pdesttileregionsize, psourcetiledata, flags);
        }
    }
    pub unsafe fn ResizeTilePool<P0>(&self, ptilepool: P0, newsizeinbytes: u64) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::d3d11::ID3D11Buffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).ResizeTilePool)(windows_core::Interface::as_raw(self), ptilepool.param().abi(), newsizeinbytes) }
    }
    pub unsafe fn TiledResourceBarrier<P0, P1>(&self, ptiledresourceorviewaccessbeforebarrier: P0, ptiledresourceorviewaccessafterbarrier: P1)
    where
        P0: windows_core::Param<super::d3d11::ID3D11DeviceChild>,
        P1: windows_core::Param<super::d3d11::ID3D11DeviceChild>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).TiledResourceBarrier)(windows_core::Interface::as_raw(self), ptiledresourceorviewaccessbeforebarrier.param().abi(), ptiledresourceorviewaccessafterbarrier.param().abi());
        }
    }
    pub unsafe fn IsAnnotationEnabled(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsAnnotationEnabled)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetMarkerInt<P0>(&self, plabel: P0, data: i32)
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetMarkerInt)(windows_core::Interface::as_raw(self), plabel.param().abi(), data);
        }
    }
    pub unsafe fn BeginEventInt<P0>(&self, plabel: P0, data: i32)
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).BeginEventInt)(windows_core::Interface::as_raw(self), plabel.param().abi(), data);
        }
    }
    pub unsafe fn EndEvent(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).EndEvent)(windows_core::Interface::as_raw(self));
        }
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1"))]
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11DeviceContext2_Vtbl {
    pub base__: super::d3d11_1::ID3D11DeviceContext1_Vtbl,
    pub UpdateTileMappings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const D3D11_TILED_RESOURCE_COORDINATE, *const D3D11_TILE_REGION_SIZE, *mut core::ffi::c_void, u32, *const u32, *const u32, *const u32, u32) -> windows_core::HRESULT,
    pub CopyTileMappings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_TILED_RESOURCE_COORDINATE, *mut core::ffi::c_void, *const D3D11_TILED_RESOURCE_COORDINATE, *const D3D11_TILE_REGION_SIZE, u32) -> windows_core::HRESULT,
    pub CopyTiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_TILED_RESOURCE_COORDINATE, *const D3D11_TILE_REGION_SIZE, *mut core::ffi::c_void, u64, u32),
    pub UpdateTiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D3D11_TILED_RESOURCE_COORDINATE, *const D3D11_TILE_REGION_SIZE, *const core::ffi::c_void, u32),
    pub ResizeTilePool: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub TiledResourceBarrier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
    pub IsAnnotationEnabled: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub SetMarkerInt: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32),
    pub BeginEventInt: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32),
    pub EndEvent: unsafe extern "system" fn(*mut core::ffi::c_void),
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat", feature = "Win32_windef"))]
pub trait ID3D11DeviceContext2_Impl: super::d3d11_1::ID3D11DeviceContext1_Impl {
    fn UpdateTileMappings(&self, ptiledresource: windows_core::Ref<super::d3d11::ID3D11Resource>, numtiledresourceregions: u32, ptiledresourceregionstartcoordinates: *const D3D11_TILED_RESOURCE_COORDINATE, ptiledresourceregionsizes: *const D3D11_TILE_REGION_SIZE, ptilepool: windows_core::Ref<super::d3d11::ID3D11Buffer>, numranges: u32, prangeflags: *const u32, ptilepoolstartoffsets: *const u32, prangetilecounts: *const u32, flags: u32) -> windows_core::Result<()>;
    fn CopyTileMappings(&self, pdesttiledresource: windows_core::Ref<super::d3d11::ID3D11Resource>, pdestregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, psourcetiledresource: windows_core::Ref<super::d3d11::ID3D11Resource>, psourceregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, flags: u32) -> windows_core::Result<()>;
    fn CopyTiles(&self, ptiledresource: windows_core::Ref<super::d3d11::ID3D11Resource>, ptileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, pbuffer: windows_core::Ref<super::d3d11::ID3D11Buffer>, bufferstartoffsetinbytes: u64, flags: u32);
    fn UpdateTiles(&self, pdesttiledresource: windows_core::Ref<super::d3d11::ID3D11Resource>, pdesttileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, pdesttileregionsize: *const D3D11_TILE_REGION_SIZE, psourcetiledata: *const core::ffi::c_void, flags: u32);
    fn ResizeTilePool(&self, ptilepool: windows_core::Ref<super::d3d11::ID3D11Buffer>, newsizeinbytes: u64) -> windows_core::Result<()>;
    fn TiledResourceBarrier(&self, ptiledresourceorviewaccessbeforebarrier: windows_core::Ref<super::d3d11::ID3D11DeviceChild>, ptiledresourceorviewaccessafterbarrier: windows_core::Ref<super::d3d11::ID3D11DeviceChild>);
    fn IsAnnotationEnabled(&self) -> windows_core::BOOL;
    fn SetMarkerInt(&self, plabel: &windows_core::PCWSTR, data: i32);
    fn BeginEventInt(&self, plabel: &windows_core::PCWSTR, data: i32);
    fn EndEvent(&self);
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat", feature = "Win32_windef"))]
impl ID3D11DeviceContext2_Vtbl {
    pub const fn new<Identity: ID3D11DeviceContext2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UpdateTileMappings<Identity: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptiledresource: *mut core::ffi::c_void, numtiledresourceregions: u32, ptiledresourceregionstartcoordinates: *const D3D11_TILED_RESOURCE_COORDINATE, ptiledresourceregionsizes: *const D3D11_TILE_REGION_SIZE, ptilepool: *mut core::ffi::c_void, numranges: u32, prangeflags: *const u32, ptilepoolstartoffsets: *const u32, prangetilecounts: *const u32, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext2_Impl::UpdateTileMappings(this, core::mem::transmute_copy(&ptiledresource), core::mem::transmute_copy(&numtiledresourceregions), core::mem::transmute_copy(&ptiledresourceregionstartcoordinates), core::mem::transmute_copy(&ptiledresourceregionsizes), core::mem::transmute_copy(&ptilepool), core::mem::transmute_copy(&numranges), core::mem::transmute_copy(&prangeflags), core::mem::transmute_copy(&ptilepoolstartoffsets), core::mem::transmute_copy(&prangetilecounts), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn CopyTileMappings<Identity: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesttiledresource: *mut core::ffi::c_void, pdestregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, psourcetiledresource: *mut core::ffi::c_void, psourceregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext2_Impl::CopyTileMappings(this, core::mem::transmute_copy(&pdesttiledresource), core::mem::transmute_copy(&pdestregionstartcoordinate), core::mem::transmute_copy(&psourcetiledresource), core::mem::transmute_copy(&psourceregionstartcoordinate), core::mem::transmute_copy(&ptileregionsize), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn CopyTiles<Identity: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptiledresource: *mut core::ffi::c_void, ptileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, pbuffer: *mut core::ffi::c_void, bufferstartoffsetinbytes: u64, flags: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext2_Impl::CopyTiles(this, core::mem::transmute_copy(&ptiledresource), core::mem::transmute_copy(&ptileregionstartcoordinate), core::mem::transmute_copy(&ptileregionsize), core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&bufferstartoffsetinbytes), core::mem::transmute_copy(&flags));
            }
        }
        unsafe extern "system" fn UpdateTiles<Identity: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdesttiledresource: *mut core::ffi::c_void, pdesttileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, pdesttileregionsize: *const D3D11_TILE_REGION_SIZE, psourcetiledata: *const core::ffi::c_void, flags: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext2_Impl::UpdateTiles(this, core::mem::transmute_copy(&pdesttiledresource), core::mem::transmute_copy(&pdesttileregionstartcoordinate), core::mem::transmute_copy(&pdesttileregionsize), core::mem::transmute_copy(&psourcetiledata), core::mem::transmute_copy(&flags));
            }
        }
        unsafe extern "system" fn ResizeTilePool<Identity: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptilepool: *mut core::ffi::c_void, newsizeinbytes: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext2_Impl::ResizeTilePool(this, core::mem::transmute_copy(&ptilepool), core::mem::transmute_copy(&newsizeinbytes)).into()
            }
        }
        unsafe extern "system" fn TiledResourceBarrier<Identity: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptiledresourceorviewaccessbeforebarrier: *mut core::ffi::c_void, ptiledresourceorviewaccessafterbarrier: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext2_Impl::TiledResourceBarrier(this, core::mem::transmute_copy(&ptiledresourceorviewaccessbeforebarrier), core::mem::transmute_copy(&ptiledresourceorviewaccessafterbarrier));
            }
        }
        unsafe extern "system" fn IsAnnotationEnabled<Identity: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext2_Impl::IsAnnotationEnabled(this)
            }
        }
        unsafe extern "system" fn SetMarkerInt<Identity: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plabel: windows_core::PCWSTR, data: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext2_Impl::SetMarkerInt(this, core::mem::transmute(&plabel), core::mem::transmute_copy(&data));
            }
        }
        unsafe extern "system" fn BeginEventInt<Identity: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plabel: windows_core::PCWSTR, data: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext2_Impl::BeginEventInt(this, core::mem::transmute(&plabel), core::mem::transmute_copy(&data));
            }
        }
        unsafe extern "system" fn EndEvent<Identity: ID3D11DeviceContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceContext2_Impl::EndEvent(this);
            }
        }
        Self {
            base__: super::d3d11_1::ID3D11DeviceContext1_Vtbl::new::<Identity, OFFSET>(),
            UpdateTileMappings: UpdateTileMappings::<Identity, OFFSET>,
            CopyTileMappings: CopyTileMappings::<Identity, OFFSET>,
            CopyTiles: CopyTiles::<Identity, OFFSET>,
            UpdateTiles: UpdateTiles::<Identity, OFFSET>,
            ResizeTilePool: ResizeTilePool::<Identity, OFFSET>,
            TiledResourceBarrier: TiledResourceBarrier::<Identity, OFFSET>,
            IsAnnotationEnabled: IsAnnotationEnabled::<Identity, OFFSET>,
            SetMarkerInt: SetMarkerInt::<Identity, OFFSET>,
            BeginEventInt: BeginEventInt::<Identity, OFFSET>,
            EndEvent: EndEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11DeviceContext2 as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11DeviceChild as windows_core::Interface>::IID || iid == &<super::d3d11::ID3D11DeviceContext as windows_core::Interface>::IID || iid == &<super::d3d11_1::ID3D11DeviceContext1 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3d11_1", feature = "Win32_d3dcommon", feature = "Win32_dxgiformat", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ID3D11DeviceContext2 {}
