#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IJsonArray(pub *mut ::core::ffi::c_void);
pub struct IJsonArrayStatics(pub *mut ::core::ffi::c_void);
pub struct IJsonErrorStatics2(pub *mut ::core::ffi::c_void);
pub struct IJsonObject(pub *mut ::core::ffi::c_void);
pub struct IJsonObjectStatics(pub *mut ::core::ffi::c_void);
pub struct IJsonObjectWithDefaultValues(pub *mut ::core::ffi::c_void);
pub struct IJsonValue(pub *mut ::core::ffi::c_void);
pub struct IJsonValueStatics(pub *mut ::core::ffi::c_void);
pub struct IJsonValueStatics2(pub *mut ::core::ffi::c_void);
pub struct JsonArray(i32);
pub struct JsonError(i32);
pub struct JsonErrorStatus(i32);
pub struct JsonObject(i32);
pub struct JsonValue(i32);
pub struct JsonValueType(i32);
