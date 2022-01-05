#[cfg(feature = "implement_exclusive")]
pub trait IJsonArrayImpl: Sized + IJsonValueImpl {
    fn GetObjectAt(&self, index: u32) -> ::windows::core::Result<JsonObject>;
    fn GetArrayAt(&self, index: u32) -> ::windows::core::Result<JsonArray>;
    fn GetStringAt(&self, index: u32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetNumberAt(&self, index: u32) -> ::windows::core::Result<f64>;
    fn GetBooleanAt(&self, index: u32) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IJsonArrayStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<JsonArray>;
    fn TryParse(&self, input: &::windows::core::HSTRING, result: &mut ::core::option::Option<JsonArray>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IJsonErrorStatics2Impl: Sized {
    fn GetJsonStatus(&self, hresult: i32) -> ::windows::core::Result<JsonErrorStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IJsonObjectImpl: Sized + IJsonValueImpl {
    fn GetNamedValue(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<JsonValue>;
    fn SetNamedValue(&self, name: &::windows::core::HSTRING, value: &::core::option::Option<IJsonValue>) -> ::windows::core::Result<()>;
    fn GetNamedObject(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<JsonObject>;
    fn GetNamedArray(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<JsonArray>;
    fn GetNamedString(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetNamedNumber(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<f64>;
    fn GetNamedBoolean(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IJsonObjectStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<JsonObject>;
    fn TryParse(&self, input: &::windows::core::HSTRING, result: &mut ::core::option::Option<JsonObject>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IJsonObjectWithDefaultValuesImpl: Sized + IJsonObjectImpl + IJsonValueImpl {
    fn GetNamedValueOrDefault(&self, name: &::windows::core::HSTRING, defaultvalue: &::core::option::Option<JsonValue>) -> ::windows::core::Result<JsonValue>;
    fn GetNamedObjectOrDefault(&self, name: &::windows::core::HSTRING, defaultvalue: &::core::option::Option<JsonObject>) -> ::windows::core::Result<JsonObject>;
    fn GetNamedStringOrDefault(&self, name: &::windows::core::HSTRING, defaultvalue: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetNamedArrayOrDefault(&self, name: &::windows::core::HSTRING, defaultvalue: &::core::option::Option<JsonArray>) -> ::windows::core::Result<JsonArray>;
    fn GetNamedNumberOrDefault(&self, name: &::windows::core::HSTRING, defaultvalue: f64) -> ::windows::core::Result<f64>;
    fn GetNamedBooleanOrDefault(&self, name: &::windows::core::HSTRING, defaultvalue: bool) -> ::windows::core::Result<bool>;
}
pub trait IJsonValueImpl: Sized {
    fn ValueType(&self) -> ::windows::core::Result<JsonValueType>;
    fn Stringify(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetNumber(&self) -> ::windows::core::Result<f64>;
    fn GetBoolean(&self) -> ::windows::core::Result<bool>;
    fn GetArray(&self) -> ::windows::core::Result<JsonArray>;
    fn GetObject(&self) -> ::windows::core::Result<JsonObject>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IJsonValueStaticsImpl: Sized {
    fn Parse(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<JsonValue>;
    fn TryParse(&self, input: &::windows::core::HSTRING, result: &mut ::core::option::Option<JsonValue>) -> ::windows::core::Result<bool>;
    fn CreateBooleanValue(&self, input: bool) -> ::windows::core::Result<JsonValue>;
    fn CreateNumberValue(&self, input: f64) -> ::windows::core::Result<JsonValue>;
    fn CreateStringValue(&self, input: &::windows::core::HSTRING) -> ::windows::core::Result<JsonValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IJsonValueStatics2Impl: Sized {
    fn CreateNullValue(&self) -> ::windows::core::Result<JsonValue>;
}
