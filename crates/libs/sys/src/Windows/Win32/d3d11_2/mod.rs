pub type D3D11_CHECK_MULTISAMPLE_QUALITY_LEVELS_FLAG = i32;
pub const D3D11_CHECK_MULTISAMPLE_QUALITY_LEVELS_TILED_RESOURCE: D3D11_CHECK_MULTISAMPLE_QUALITY_LEVELS_FLAG = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_PACKED_MIP_DESC {
    pub NumStandardMips: u8,
    pub NumPackedMips: u8,
    pub NumTilesForPackedMips: u32,
    pub StartTileIndexInOverallResource: u32,
}
pub const D3D11_PACKED_TILE: u32 = 4294967295;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_SUBRESOURCE_TILING {
    pub WidthInTiles: u32,
    pub HeightInTiles: u16,
    pub DepthInTiles: u16,
    pub StartTileIndexInOverallResource: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[derive(Clone, Copy, Default)]
pub struct D3D11_TILE_REGION_SIZE {
    pub NumTiles: u32,
    pub bUseBox: windows_sys::core::BOOL,
    pub Width: u32,
    pub Height: u16,
    pub Depth: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_TILE_SHAPE {
    pub WidthInTexels: u32,
    pub HeightInTexels: u32,
    pub DepthInTexels: u32,
}
