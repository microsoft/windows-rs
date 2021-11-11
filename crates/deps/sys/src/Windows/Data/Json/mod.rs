#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IJsonArray();
    fn IJsonArrayStatics();
    fn IJsonErrorStatics2();
    fn IJsonObject();
    fn IJsonObjectStatics();
    fn IJsonObjectWithDefaultValues();
    fn IJsonValue();
    fn IJsonValueStatics();
    fn IJsonValueStatics2();
    fn JsonArray();
    fn JsonError();
    fn JsonErrorStatus();
    fn JsonObject();
    fn JsonValue();
    fn JsonValueType();
}
