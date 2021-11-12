#![allow(non_snake_case, non_camel_case_types)]
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
pub struct JsonError(pub *mut ::core::ffi::c_void);
pub struct JsonErrorStatus(i32);
#[repr(transparent)]
pub struct JsonObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct JsonValue(pub *mut ::core::ffi::c_void);
pub struct JsonValueType(i32);
