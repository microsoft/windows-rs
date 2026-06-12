/// Generate an `Rc`-pointer-equal newtype around `dyn Fn(...)`. Provides
/// `new`, `Clone`, `Debug` (pointer-formatted), and `PartialEq`/`Eq`
/// based on `Rc::ptr_eq`.
#[macro_export]
#[doc(hidden)]
macro_rules! impl_rc_fn_wrapper {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident $(< $param:ident >)? (
            dyn Fn ( $($args:tt)* ) $(-> $ret:ty)?
        );
    ) => {
        $(#[$attr])*
        $vis struct $name $(< $param >)? {
            inner: ::std::rc::Rc<dyn Fn($($args)*) $(-> $ret)?>,
        }

        impl $(< $param >)? $name $(< $param >)? {
            pub fn new<__F>(f: __F) -> Self
            where
                __F: Fn($($args)*) $(-> $ret)? + 'static,
            {
                Self { inner: ::std::rc::Rc::new(f) }
            }
        }

        impl $(< $param >)? ::std::clone::Clone for $name $(< $param >)? {
            fn clone(&self) -> Self {
                Self { inner: ::std::rc::Rc::clone(&self.inner) }
            }
        }

        impl $(< $param >)? ::std::fmt::Debug for $name $(< $param >)? {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_tuple(stringify!($name))
                    .field(&::std::format_args!(
                        "{:p}",
                        ::std::rc::Rc::as_ptr(&self.inner)
                    ))
                    .finish()
            }
        }

        impl $(< $param >)? ::std::cmp::PartialEq for $name $(< $param >)? {
            fn eq(&self, other: &Self) -> bool {
                ::std::rc::Rc::ptr_eq(&self.inner, &other.inner)
            }
        }

        impl $(< $param >)? ::std::cmp::Eq for $name $(< $param >)? {}
    };
}
