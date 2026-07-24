#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NDIS_HASH_FLAGS(pub u32);
pub const NDIS_HASH_FUNCTION_MASK: i32 = 255;
pub const NDIS_HASH_IPV4: i32 = 256;
pub const NDIS_HASH_IPV6: i32 = 1024;
pub const NDIS_HASH_IPV6_EX: i32 = 2048;
pub const NDIS_HASH_TCP_IPV4: i32 = 512;
pub const NDIS_HASH_TCP_IPV6: i32 = 4096;
pub const NDIS_HASH_TCP_IPV6_EX: i32 = 8192;
pub const NDIS_HASH_TYPE_MASK: i32 = 16776960;
pub const NdisHashFunctionReserved1: i32 = 2;
pub const NdisHashFunctionReserved2: i32 = 4;
pub const NdisHashFunctionReserved3: i32 = 8;
pub const NdisHashFunctionToeplitz: i32 = 1;
