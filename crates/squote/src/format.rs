/// Formatting macro for constructing `Ident`s.
///
/// <br>
///
/// # Syntax
///
/// Syntax is copied from the [`format!`] macro, supporting both positional and
/// named arguments.
#[macro_export]
macro_rules! format_ident {
    ($($fmt:tt)*) => {
        $crate::Ident::new(format!($($fmt)*))
    };
}
