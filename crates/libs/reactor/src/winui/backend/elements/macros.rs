//! Macro helpers for per-element prop dispatch.
//!
//! The `element_set_prop!` macro generates a `set_prop` function for a single
//! element type that handles both value-set and value-unset from a single
//! declaration per prop. This structurally guarantees that every prop setter
//! has a corresponding default/reset — the class of bugs where `PropValue::Unset`
//! is silently swallowed by a catch-all cannot occur for macro-declared props.

/// Generates a per-element `set_prop` function from a declarative prop table.
///
/// Each entry declares a prop with its value type, setter expression, and
/// default value. The macro expands to a `match prop` with one arm per prop;
/// within each arm, the value is extracted (or the default is used for `Unset`).
///
/// The generated function returns `Option<windows_core::Result<()>>`:
/// - `Some(Ok(()))` — prop was handled successfully
/// - `Some(Err(e))` — prop was handled but a COM error occurred
/// - `None` — prop is not handled by this element (fall through to common)
///
/// # Supported kinds
///
/// - `bool` — extracts from `PropValue::Bool`
/// - `f64` — extracts from `PropValue::F64`
/// - `str` — extracts from `PropValue::Str` (as `&str`)
/// - `u16` — extracts from `PropValue::U16`
/// - `i32` — extracts from `PropValue::I32`
///
/// # Example
///
/// ```ignore
/// element_set_prop! {
///     pub(super) fn set_prop(tb: &Xaml::TextBlock) {
///         bool IsTextSelectionEnabled(v) => tb.put_IsTextSelectionEnabled(v), default: false;
///         f64  FontSize(v)               => tb.put_FontSize(v),               default: 14.0;
///         str  Text(v)                   => tb.put_Text(v),                   default: "";
///     }
/// }
/// ```
#[allow(unused_macros)]
macro_rules! element_set_prop {
    (
        $vis:vis fn $fn_name:ident($h:ident: &$HType:ty) {
            $( $kind:ident $prop_name:ident($v:ident) => $setter:expr, default: $default:expr; )*
        }
    ) => {
        $vis fn $fn_name(
            $h: &$HType,
            prop: crate::core::backend::Prop,
            value: &crate::core::backend::PropValue,
        ) -> Option<windows_core::Result<()>> {
            use crate::core::backend::{Prop, PropValue};
            match prop {
                $(
                    Prop::$prop_name => {
                        let $v = element_set_prop!(@extract $kind, value, $default);
                        Some($setter)
                    }
                )*
                _ => None,
            }
        }
    };
    // Value extraction: returns the typed value or the default for Unset.
    // Returns `None` (not handled) for type mismatches (wrong PropValue variant).
    (@extract bool, $value:expr, $default:expr) => {
        match $value {
            PropValue::Bool(b) => *b,
            PropValue::Unset => $default,
            _ => return None,
        }
    };
    (@extract f64, $value:expr, $default:expr) => {
        match $value {
            PropValue::F64(f) => *f,
            PropValue::Unset => $default,
            _ => return None,
        }
    };
    (@extract str, $value:expr, $default:expr) => {
        match $value {
            PropValue::Str(s) => s.as_str(),
            PropValue::Unset => $default,
            _ => return None,
        }
    };
    (@extract u16, $value:expr, $default:expr) => {
        match $value {
            PropValue::U16(n) => *n,
            PropValue::Unset => $default,
            _ => return None,
        }
    };
    (@extract i32, $value:expr, $default:expr) => {
        match $value {
            PropValue::I32(n) => *n,
            PropValue::Unset => $default,
            _ => return None,
        }
    };
}
