#[inline]
pub unsafe fn RtlExtendCorrelationVector(correlationvector: *mut CORRELATION_VECTOR) -> u32 {
    ::windows_targets::link!("ntdll.dll" "system" fn RtlExtendCorrelationVector(correlationvector : *mut CORRELATION_VECTOR) -> u32);
    RtlExtendCorrelationVector(correlationvector)
}
#[inline]
pub unsafe fn RtlIncrementCorrelationVector(correlationvector: *mut CORRELATION_VECTOR) -> u32 {
    ::windows_targets::link!("ntdll.dll" "system" fn RtlIncrementCorrelationVector(correlationvector : *mut CORRELATION_VECTOR) -> u32);
    RtlIncrementCorrelationVector(correlationvector)
}
#[inline]
pub unsafe fn RtlInitializeCorrelationVector(correlationvector: *mut CORRELATION_VECTOR, version: i32, guid: ::core::option::Option<*const ::windows_core::GUID>) -> u32 {
    ::windows_targets::link!("ntdll.dll" "system" fn RtlInitializeCorrelationVector(correlationvector : *mut CORRELATION_VECTOR, version : i32, guid : *const ::windows_core::GUID) -> u32);
    RtlInitializeCorrelationVector(correlationvector, version, ::core::mem::transmute(guid.unwrap_or(::std::ptr::null())))
}
#[inline]
pub unsafe fn RtlValidateCorrelationVector(vector: *const CORRELATION_VECTOR) -> u32 {
    ::windows_targets::link!("ntdll.dll" "system" fn RtlValidateCorrelationVector(vector : *const CORRELATION_VECTOR) -> u32);
    RtlValidateCorrelationVector(vector)
}
pub const RTL_CORRELATION_VECTOR_STRING_LENGTH: u32 = 129u32;
pub const RTL_CORRELATION_VECTOR_V1_LENGTH: u32 = 64u32;
pub const RTL_CORRELATION_VECTOR_V1_PREFIX_LENGTH: u32 = 16u32;
pub const RTL_CORRELATION_VECTOR_V2_LENGTH: u32 = 128u32;
pub const RTL_CORRELATION_VECTOR_V2_PREFIX_LENGTH: u32 = 22u32;
#[repr(C)]
pub struct CORRELATION_VECTOR {
    pub Version: u8,
    pub Vector: [u8; 129],
}
impl ::core::marker::Copy for CORRELATION_VECTOR {}
impl ::core::clone::Clone for CORRELATION_VECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CORRELATION_VECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CORRELATION_VECTOR").field("Version", &self.Version).field("Vector", &self.Vector).finish()
    }
}
impl ::windows_core::TypeKind for CORRELATION_VECTOR {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for CORRELATION_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Vector == other.Vector
    }
}
impl ::core::cmp::Eq for CORRELATION_VECTOR {}
impl ::core::default::Default for CORRELATION_VECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
