#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct TextPatternRangeEndpoint(pub i32);
impl TextPatternRangeEndpoint {
    pub const Start: TextPatternRangeEndpoint = TextPatternRangeEndpoint(0i32);
    pub const End: TextPatternRangeEndpoint = TextPatternRangeEndpoint(1i32);
}
#[repr(transparent)]
pub struct TextUnit(pub i32);
impl TextUnit {
    pub const Character: TextUnit = TextUnit(0i32);
    pub const Format: TextUnit = TextUnit(1i32);
    pub const Word: TextUnit = TextUnit(2i32);
    pub const Line: TextUnit = TextUnit(3i32);
    pub const Paragraph: TextUnit = TextUnit(4i32);
    pub const Page: TextUnit = TextUnit(5i32);
    pub const Document: TextUnit = TextUnit(6i32);
}
