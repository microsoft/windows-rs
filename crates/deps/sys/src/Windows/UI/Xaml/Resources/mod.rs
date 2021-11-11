#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CustomXamlResourceLoader();
    fn ICustomXamlResourceLoader();
    fn ICustomXamlResourceLoaderFactory();
    fn ICustomXamlResourceLoaderOverrides();
    fn ICustomXamlResourceLoaderStatics();
}
