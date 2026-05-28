//! Trait for converting heterogeneous tuples into `Vec<Element>`.
//!
//! This enables writing `vstack((title, body, footer))` without macros,
//! where each tuple element can be a different type that implements
//! `Into<Element>`.

use super::element::Element;

/// Converts a collection of items into a `Vec<Element>`.
///
/// Implemented for:
/// - Tuples of up to 12 elements, each `Into<Element>` (heterogeneous lists)
/// - `Vec<Element>` (pre-built dynamic lists)
/// - Fixed-size arrays `[T; N]` where `T: Into<Element>`
pub trait IntoElements {
    fn into_elements(self) -> Vec<Element>;
}

// Vec<Element> passthrough
impl IntoElements for Vec<Element> {
    fn into_elements(self) -> Vec<Element> {
        self
    }
}

// Empty tuple = no children
impl IntoElements for () {
    fn into_elements(self) -> Vec<Element> {
        Vec::new()
    }
}

// Fixed-size arrays of Into<Element>
impl<T: Into<Element>, const N: usize> IntoElements for [T; N] {
    fn into_elements(self) -> Vec<Element> {
        self.into_iter().map(Into::into).collect()
    }
}

// Tuple impls — each element independently implements Into<Element>.
macro_rules! impl_into_elements_for_tuple {
    ($($idx:tt : $T:ident),+) => {
        impl<$($T: Into<Element>),+> IntoElements for ($($T,)+) {
            fn into_elements(self) -> Vec<Element> {
                vec![$(self.$idx.into()),+]
            }
        }
    };
}

impl_into_elements_for_tuple!(0: A);
impl_into_elements_for_tuple!(0: A, 1: B);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I2);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I2, 9: J);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I2, 9: J, 10: K);
impl_into_elements_for_tuple!(0: A, 1: B, 2: C, 3: D, 4: E, 5: F, 6: G, 7: H, 8: I2, 9: J, 10: K, 11: L);
