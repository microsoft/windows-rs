#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Com_CallObj`*"]
    pub fn CoGetInterceptor();
    #[doc = "*Required features: `Win32_System_Com_CallObj`*"]
    pub fn CoGetInterceptorFromTypeInfo();
}
