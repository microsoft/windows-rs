/// Includes the generating bindings into the current context.
#[macro_export]
macro_rules! include_bindings {
    () => {
        ::std::include!(::std::concat!(::std::env!("OUT_DIR"), "/windows.rs"));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! demand_load {
    ( $( $library:literal {
        $(fn $sym:ident ( $( $param: ident : $pty: ty ),* $(,)? ) -> $rt: ty;)*
    } )* ) => {
        $($(
            #[allow(non_snake_case)]
            unsafe fn $sym( $( $param: $pty ),* ) -> ::std::result::Result<$rt, $crate::ErrorCode> {
                static ONCE: ::std::sync::Once = ::std::sync::Once::new();
                static mut VALUE: ::std::mem::MaybeUninit<::std::result::Result<$crate::RawPtr, $crate::ErrorCode>> =
                    ::std::mem::MaybeUninit::uninit();
                const LOAD_LIBRARY_SEARCH_SYSTEM32: u32 = 0x0000_0800;

                ONCE.call_once(|| {
                    VALUE = ::std::mem::MaybeUninit::new(
                        $crate::delay_load($library, ::std::stringify!($sym), LOAD_LIBRARY_SEARCH_SYSTEM32)
                    )
                });

                // transmute() doesn't work on generic types, as you can't constrain to a
                // function pointer, so it must be done here outside load_proc().
                type FnPtr = extern "system" fn ( $( $param: $pty ),* ) -> $rt;
                let f = ::std::mem::transmute::<$crate::RawPtr, FnPtr>(VALUE.assume_init()?);
                ::std::result::Result::Ok( (f)( $( $param ),* ) )
            }
        )*)*
    };
}
