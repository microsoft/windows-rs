#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IXsltProcessor();
    fn IXsltProcessor2();
    fn IXsltProcessorFactory();
    fn XsltProcessor();
}
