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
    pub const Unknown: JsonErrorStatus = JsonErrorStatus(0i32);
    pub const InvalidJsonString: JsonErrorStatus = JsonErrorStatus(1i32);
    pub const InvalidJsonNumber: JsonErrorStatus = JsonErrorStatus(2i32);
    pub const JsonValueNotFound: JsonErrorStatus = JsonErrorStatus(3i32);
    pub const ImplementationLimit: JsonErrorStatus = JsonErrorStatus(4i32);
}
#[repr(transparent)]
pub struct JsonObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct JsonValue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct JsonValueType(pub i32);
impl JsonValueType {
    pub const Null: JsonValueType = JsonValueType(0i32);
    pub const Boolean: JsonValueType = JsonValueType(1i32);
    pub const Number: JsonValueType = JsonValueType(2i32);
    pub const String: JsonValueType = JsonValueType(3i32);
    pub const Array: JsonValueType = JsonValueType(4i32);
    pub const Object: JsonValueType = JsonValueType(5i32);
}
