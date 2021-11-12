#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IJsonArray(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJsonArrayStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJsonErrorStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJsonObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJsonObjectStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJsonObjectWithDefaultValues(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJsonValue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJsonValueStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJsonValueStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct JsonArray(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct JsonErrorStatus(pub i32);
impl JsonErrorStatus {
    pub const Unknown: Self = Self(0i32);
    pub const InvalidJsonString: Self = Self(1i32);
    pub const InvalidJsonNumber: Self = Self(2i32);
    pub const JsonValueNotFound: Self = Self(3i32);
    pub const ImplementationLimit: Self = Self(4i32);
}
#[repr(transparent)]
pub struct JsonObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct JsonValue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct JsonValueType(pub i32);
impl JsonValueType {
    pub const Null: Self = Self(0i32);
    pub const Boolean: Self = Self(1i32);
    pub const Number: Self = Self(2i32);
    pub const String: Self = Self(3i32);
    pub const Array: Self = Self(4i32);
    pub const Object: Self = Self(5i32);
}
