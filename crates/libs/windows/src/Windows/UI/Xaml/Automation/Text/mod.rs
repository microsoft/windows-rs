#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'UI_Xaml_Automation_Text'*"]
#[repr(transparent)]
pub struct TextPatternRangeEndpoint(pub i32);
impl TextPatternRangeEndpoint {
    pub const Start: Self = Self(0i32);
    pub const End: Self = Self(1i32);
}
impl ::core::marker::Copy for TextPatternRangeEndpoint {}
impl ::core::clone::Clone for TextPatternRangeEndpoint {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TextPatternRangeEndpoint {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TextPatternRangeEndpoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TextPatternRangeEndpoint {}
impl ::core::fmt::Debug for TextPatternRangeEndpoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextPatternRangeEndpoint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TextPatternRangeEndpoint {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.Text.TextPatternRangeEndpoint;i4)");
}
impl ::windows::core::DefaultType for TextPatternRangeEndpoint {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Xaml_Automation_Text'*"]
#[repr(transparent)]
pub struct TextUnit(pub i32);
impl TextUnit {
    pub const Character: Self = Self(0i32);
    pub const Format: Self = Self(1i32);
    pub const Word: Self = Self(2i32);
    pub const Line: Self = Self(3i32);
    pub const Paragraph: Self = Self(4i32);
    pub const Page: Self = Self(5i32);
    pub const Document: Self = Self(6i32);
}
impl ::core::marker::Copy for TextUnit {}
impl ::core::clone::Clone for TextUnit {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TextUnit {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TextUnit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TextUnit {}
impl ::core::fmt::Debug for TextUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TextUnit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TextUnit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Automation.Text.TextUnit;i4)");
}
impl ::windows::core::DefaultType for TextUnit {
    type DefaultType = Self;
}
