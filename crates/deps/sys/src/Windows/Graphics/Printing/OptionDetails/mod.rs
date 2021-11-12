#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPrintBindingOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintBorderingOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintCollationOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintColorModeOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintCopiesOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintCustomItemDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintCustomItemListOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintCustomItemListOptionDetails2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintCustomItemListOptionDetails3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintCustomOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintCustomTextOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintCustomTextOptionDetails2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintCustomToggleOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintDuplexOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintHolePunchOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintItemListOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintMediaSizeOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintMediaTypeOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintNumberOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintOrientationOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintPageRangeOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintQualityOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintStapleOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTaskOptionChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTaskOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTaskOptionDetails2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTaskOptionDetailsStatic(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintTextOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintBindingOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintBorderingOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintCollationOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintColorModeOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintCopiesOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintCustomItemDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintCustomItemListOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintCustomTextOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintCustomToggleOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintDuplexOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintHolePunchOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintMediaSizeOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintMediaTypeOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintOptionStates(pub u32);
impl PrintOptionStates {
    pub const None: Self = Self(0u32);
    pub const Enabled: Self = Self(1u32);
    pub const Constrained: Self = Self(2u32);
}
impl ::core::marker::Copy for PrintOptionStates {}
impl ::core::clone::Clone for PrintOptionStates {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintOptionType(pub i32);
impl PrintOptionType {
    pub const Unknown: Self = Self(0i32);
    pub const Number: Self = Self(1i32);
    pub const Text: Self = Self(2i32);
    pub const ItemList: Self = Self(3i32);
    pub const Toggle: Self = Self(4i32);
}
impl ::core::marker::Copy for PrintOptionType {}
impl ::core::clone::Clone for PrintOptionType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintOrientationOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintPageRangeOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintQualityOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintStapleOptionDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintTaskOptionChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PrintTaskOptionDetails(pub *mut ::core::ffi::c_void);
