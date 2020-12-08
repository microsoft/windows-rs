/// Includes the generating bindings into the current context.
#[macro_export]
macro_rules! include_bindings {
    () => {
        include!(concat!(env!("OUT_DIR"), "\\windows.rs"));
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
            unsafe fn $sym( $( $param: $pty ),* ) -> std::result::Result<$rt, ErrorCode> {
                use std::{sync::Once, mem::MaybeUninit};

                static ONCE: Once = Once::new();
                static mut VALUE: MaybeUninit<std::result::Result<RawPtr, ErrorCode>> = MaybeUninit::uninit();
                const LOAD_LIBRARY_SEARCH_SYSTEM32: u32 = 0x0000_0800;

                ONCE.call_once(|| {
                    VALUE = MaybeUninit::new(
                        delay_load($library, stringify!($sym), LOAD_LIBRARY_SEARCH_SYSTEM32)
                    )
                });

                // transmute() doesn't work on generic types, as you can't constrain to a
                // function pointer, so it must be done here outside load_proc().
                type FnPtr = extern "system" fn ( $( $param: $pty ),* ) -> $rt;
                let f = std::mem::transmute::<RawPtr, FnPtr>(VALUE.assume_init()?);
                Ok( (f)( $( $param ),* ) )
            }
        )*)*
    };
}
