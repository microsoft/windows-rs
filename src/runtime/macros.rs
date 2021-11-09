#[doc(hidden)]
#[macro_export]
macro_rules! demand_load {
    ( $( $library:literal {
        $(fn $sym:ident ( $( $param: ident : $pty: ty ),* $(,)? ) -> $rt: ty;)*
    } )* ) => {
        $($(
            #[allow(non_snake_case)]
            unsafe fn $sym( $( $param: $pty ),* ) -> ::core::option::Option<$rt> {
                type FnPtr = extern "system" fn ( $( $param: $pty ),* ) -> $rt;

                static mut VALUE: ::core::sync::atomic::AtomicPtr<core::ffi::c_void> =
                ::core::sync::atomic::AtomicPtr::new(::core::ptr::null_mut());

                let mut ptr = VALUE.load(::core::sync::atomic::Ordering::Relaxed);

                if ptr.is_null() {
                    ptr = ::windows::runtime::delay_load($library.as_bytes(), ::core::stringify!($sym));

                    if ptr.is_null() {
                        return None;
                    }

                    VALUE.store(ptr, ::core::sync::atomic::Ordering::Relaxed);
                }

                let f: FnPtr = ::core::mem::transmute(ptr);
                ::core::option::Option::Some((f)($($param),*))
            }
        )*)*
    };
}
