pub const BG_COPY_FILE_ALL: u32 = 15;
pub const BG_COPY_FILE_DACL: u32 = 4;
pub const BG_COPY_FILE_GROUP: u32 = 2;
pub const BG_COPY_FILE_OWNER: u32 = 1;
pub const BG_COPY_FILE_SACL: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BG_FILE_RANGE {
    pub InitialOffset: u64,
    pub Length: u64,
}
pub const BG_LENGTH_TO_EOF: i32 = -1;
pub const BackgroundCopyManager2_0: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6d18ad12_bde3_4393_b311_099c346e6df9);
