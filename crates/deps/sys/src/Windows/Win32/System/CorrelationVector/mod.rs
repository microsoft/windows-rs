#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CORRELATION_VECTOR();
    fn RTL_CORRELATION_VECTOR_STRING_LENGTH();
    fn RTL_CORRELATION_VECTOR_V1_LENGTH();
    fn RTL_CORRELATION_VECTOR_V1_PREFIX_LENGTH();
    fn RTL_CORRELATION_VECTOR_V2_LENGTH();
    fn RTL_CORRELATION_VECTOR_V2_PREFIX_LENGTH();
    fn RtlExtendCorrelationVector();
    fn RtlIncrementCorrelationVector();
    fn RtlInitializeCorrelationVector();
    fn RtlValidateCorrelationVector();
}
