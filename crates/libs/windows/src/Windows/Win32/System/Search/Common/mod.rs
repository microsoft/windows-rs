#[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CONDITION_OPERATION(pub i32);
#[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
pub const COP_IMPLICIT: CONDITION_OPERATION = CONDITION_OPERATION(0i32);
#[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
pub const COP_EQUAL: CONDITION_OPERATION = CONDITION_OPERATION(1i32);
#[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
pub const COP_NOTEQUAL: CONDITION_OPERATION = CONDITION_OPERATION(2i32);
#[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
pub const COP_LESSTHAN: CONDITION_OPERATION = CONDITION_OPERATION(3i32);
#[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
pub const COP_GREATERTHAN: CONDITION_OPERATION = CONDITION_OPERATION(4i32);
#[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
pub const COP_LESSTHANOREQUAL: CONDITION_OPERATION = CONDITION_OPERATION(5i32);
#[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
pub const COP_GREATERTHANOREQUAL: CONDITION_OPERATION = CONDITION_OPERATION(6i32);
#[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
pub const COP_VALUE_STARTSWITH: CONDITION_OPERATION = CONDITION_OPERATION(7i32);
#[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
pub const COP_VALUE_ENDSWITH: CONDITION_OPERATION = CONDITION_OPERATION(8i32);
#[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
pub const COP_VALUE_CONTAINS: CONDITION_OPERATION = CONDITION_OPERATION(9i32);
#[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
pub const COP_VALUE_NOTCONTAINS: CONDITION_OPERATION = CONDITION_OPERATION(10i32);
#[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
pub const COP_DOSWILDCARDS: CONDITION_OPERATION = CONDITION_OPERATION(11i32);
#[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
pub const COP_WORD_EQUAL: CONDITION_OPERATION = CONDITION_OPERATION(12i32);
#[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
pub const COP_WORD_STARTSWITH: CONDITION_OPERATION = CONDITION_OPERATION(13i32);
#[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
pub const COP_APPLICATION_SPECIFIC: CONDITION_OPERATION = CONDITION_OPERATION(14i32);
impl ::core::marker::Copy for CONDITION_OPERATION {}
impl ::core::clone::Clone for CONDITION_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CONDITION_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CONDITION_OPERATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for CONDITION_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONDITION_OPERATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CONDITION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
pub const CT_AND_CONDITION: CONDITION_TYPE = CONDITION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
pub const CT_OR_CONDITION: CONDITION_TYPE = CONDITION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
pub const CT_NOT_CONDITION: CONDITION_TYPE = CONDITION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
pub const CT_LEAF_CONDITION: CONDITION_TYPE = CONDITION_TYPE(3i32);
impl ::core::marker::Copy for CONDITION_TYPE {}
impl ::core::clone::Clone for CONDITION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CONDITION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CONDITION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for CONDITION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONDITION_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
