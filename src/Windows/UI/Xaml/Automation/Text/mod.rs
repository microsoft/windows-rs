#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TextPatternRangeEndpoint(pub i32);
impl TextPatternRangeEndpoint {
    pub const Start: TextPatternRangeEndpoint = TextPatternRangeEndpoint(0i32);
    pub const End: TextPatternRangeEndpoint = TextPatternRangeEndpoint(1i32);
}
impl ::core::convert::From<i32> for TextPatternRangeEndpoint {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TextPatternRangeEndpoint {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TextPatternRangeEndpoint {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.Text.TextPatternRangeEndpoint;i4)");
}
impl ::windows::core::DefaultType for TextPatternRangeEndpoint {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for TextUnit {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TextUnit {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TextUnit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.Text.TextUnit;i4)");
}
impl ::windows::core::DefaultType for TextUnit {
    type DefaultType = Self;
}
