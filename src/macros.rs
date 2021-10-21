/// Includes the generated bindings into the current context.
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! include_bindings {
    () => {
        ::std::include!(::std::concat!(::std::env!("OUT_DIR"), "/windows.rs"));
    };
}
