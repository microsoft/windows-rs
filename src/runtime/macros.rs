#[doc(hidden)]
#[macro_export]
macro_rules! demand_load {
    ( $( $library:literal {
        $(fn $sym:ident ( $( $param: ident : $pty: ty ),* $(,)? ) -> $rt: ty;)*
    } )* ) => {
        $($(
            #[allow(non_snake_case)]
            unsafe fn $sym( $( $param: $pty ),* ) -> ::core::result::Result<$rt, ::windows::runtime::HRESULT> {
                static ONCE: ::std::sync::Once = ::std::sync::Once::new();
                static mut VALUE: ::core::mem::MaybeUninit<::core::result::Result<::windows::runtime::RawPtr, ::windows::runtime::HRESULT>> =
                    ::core::mem::MaybeUninit::uninit();

                ONCE.call_once(|| {
                    VALUE = ::core::mem::MaybeUninit::new(
                        ::windows::runtime::delay_load($library, ::core::stringify!($sym))
                    )
                });

                // transmute() doesn't work on generic types, as you can't constrain to a
                // function pointer, so it must be done here outside load_proc().
                type FnPtr = extern "system" fn ( $( $param: $pty ),* ) -> $rt;
                let f = ::core::mem::transmute::<::windows::runtime::RawPtr, FnPtr>(VALUE.assume_init()?);
                ::core::result::Result::Ok( (f)( $( $param ),* ) )
            }
        )*)*
    };
}
