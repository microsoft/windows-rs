//! This crate provides the [`quote!`] macro for turning Rust syntax tree data
//! structures into tokens of source code.
//!
//! [`quote!`]: macro.quote.html
//!
//! The idea of quasi-quoting is that we write *code* that we treat as *data*.
//! Within the `quote!` macro, we can write what looks like code to our text
//! editor or IDE. We get all the benefits of the editor's brace matching,
//! syntax highlighting, indentation, and maybe autocompletion. But rather than
//! compiling that as code into the current crate, we can treat it as data, pass
//! it around, mutate it, and eventually hand it back to the compiler as tokens
//! to compile into the macro caller's crate.
//!
//! This crate is motivated by the procedural macro use case, but is a
//! general-purpose Rust quasi-quoting library and is not specific to procedural
//! macros.
//!
//! ```toml
//! [dependencies]
//! squote = "1.0"
//! ```

#![forbid(unsafe_code)]

mod format;
mod to_tokens;
mod token_stream;

// Not public API.
#[doc(hidden)]
#[path = "runtime.rs"]
pub mod __private;

pub use crate::to_tokens::ToTokens;
pub use crate::token_stream::{Delimiter, Ident, Literal, TokenStream};

/// The whole point.
///
/// Performs variable interpolation against the input and produces it as
/// [`TokenStream`].
///
/// # Interpolation
///
/// Variable interpolation is done with `#var` (similar to `$var` in
/// `macro_rules!` macros). This grabs the `var` variable that is currently in
/// scope and inserts it in that location in the output tokens. Any type
/// implementing the [`ToTokens`] trait can be interpolated. This includes most
/// Rust primitive types.
///
/// [`ToTokens`]: trait.ToTokens.html
///
/// Repetition is done using `#(...)*` or `#(...),*` again similar to
/// `macro_rules!`. This iterates through the elements of any variable
/// interpolated within the repetition and inserts a copy of the repetition body
/// for each one. The variables in an interpolation may be a `Vec`, slice,
/// `BTreeSet`, or any `Iterator`.
///
/// - `#(#var)*` — no separators
/// - `#(#var),*` — the character before the asterisk is used as a separator
/// - `#( struct #var; )*` — the repetition can contain other tokens
/// - `#( #k => println!("{}", #v), )*` — even multiple interpolations
#[macro_export]
macro_rules! quote {
    () => {
        $crate::TokenStream::new()
    };
    ($($tt:tt)*) => {{
        let mut _s = $crate::TokenStream::new();
        $crate::quote_each_token!(_s $($tt)*);
        _s
    }};
}

// Extract the names of all #metavariables and pass them to the $call macro.
//
// in:   pounded_var_names!(then!(...) a #b c #( #d )* #e)
// out:  then!(... b);
//       then!(... d);
//       then!(... e);
#[macro_export]
#[doc(hidden)]
macro_rules! pounded_var_names {
    ($call:ident! $extra:tt $($tts:tt)*) => {
        $crate::pounded_var_names_with_context!($call! $extra
            (@ $($tts)*)
            ($($tts)* @)
        )
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! pounded_var_names_with_context {
    ($call:ident! $extra:tt ($($b1:tt)*) ($($curr:tt)*)) => {
        $(
            $crate::pounded_var_with_context!($call! $extra $b1 $curr);
        )*
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! pounded_var_with_context {
    ($call:ident! $extra:tt $b1:tt ( $($inner:tt)* )) => {
        $crate::pounded_var_names!($call! $extra $($inner)*);
    };

    ($call:ident! $extra:tt $b1:tt [ $($inner:tt)* ]) => {
        $crate::pounded_var_names!($call! $extra $($inner)*);
    };

    ($call:ident! $extra:tt $b1:tt { $($inner:tt)* }) => {
        $crate::pounded_var_names!($call! $extra $($inner)*);
    };

    ($call:ident!($($extra:tt)*) # $var:ident) => {
        $crate::$call!($($extra)* $var);
    };

    ($call:ident! $extra:tt $b1:tt $curr:tt) => {};
}

#[macro_export]
#[doc(hidden)]
macro_rules! quote_bind_into_iter {
    ($has_iter:ident $var:ident) => {
        // `mut` may be unused if $var occurs multiple times in the list.
        #[allow(unused_mut)]
        let (mut $var, i) = $var.quote_into_iter();
        let $has_iter = $has_iter | i;
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! quote_bind_next_or_break {
    ($var:ident) => {
        let $var = match $var.next() {
            Some(_x) => $crate::__private::RepInterp(_x),
            None => break,
        };
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! quote_each_token {
    ($tokens:ident $($tts:tt)*) => {
        $crate::quote_tokens_with_context!($tokens
            (@ @ @ @ @ @ $($tts)*)
            (@ @ @ @ @ $($tts)* @)
            (@ @ @ @ $($tts)* @ @)
            (@ @ @ $(($tts))* @ @ @)
            (@ @ $($tts)* @ @ @ @)
            (@ $($tts)* @ @ @ @ @)
            ($($tts)* @ @ @ @ @ @)
        );
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! quote_tokens_with_context {
    ($tokens:ident
        ($($b3:tt)*) ($($b2:tt)*) ($($b1:tt)*)
        ($($curr:tt)*)
        ($($a1:tt)*) ($($a2:tt)*) ($($a3:tt)*)
    ) => {
        $(
            $crate::quote_token_with_context!($tokens $b3 $b2 $b1 $curr $a1 $a2 $a3);
        )*
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! quote_token_with_context {
    ($tokens:ident $b3:tt $b2:tt $b1:tt @ $a1:tt $a2:tt $a3:tt) => {};

    ($tokens:ident $b3:tt $b2:tt $b1:tt (#) ( $($inner:tt)* ) * $a3:tt) => {{
        use $crate::__private::ext::*;
        let has_iter = $crate::__private::ThereIsNoIteratorInRepetition;
        $crate::pounded_var_names!(quote_bind_into_iter!(has_iter) () $($inner)*);
        let _: $crate::__private::HasIterator = has_iter;
        // This is `while true` instead of `loop` because if there are no
        // iterators used inside of this repetition then the body would not
        // contain any `break`, so the compiler would emit unreachable code
        // warnings on anything below the loop. We use has_iter to detect and
        // fail to compile when there are no iterators, so here we just work
        // around the unneeded extra warning.
        while true {
            $crate::pounded_var_names!(quote_bind_next_or_break!() () $($inner)*);
            $crate::quote_each_token!($tokens $($inner)*);
        }
    }};
    ($tokens:ident $b3:tt $b2:tt # (( $($inner:tt)* )) * $a2:tt $a3:tt) => {};
    ($tokens:ident $b3:tt # ( $($inner:tt)* ) (*) $a1:tt $a2:tt $a3:tt) => {};

    ($tokens:ident $b3:tt $b2:tt $b1:tt (#) ( $($inner:tt)* ) $sep:tt *) => {{
        use $crate::__private::ext::*;
        let mut _i = 0usize;
        let has_iter = $crate::__private::ThereIsNoIteratorInRepetition;
        $crate::pounded_var_names!(quote_bind_into_iter!(has_iter) () $($inner)*);
        let _: $crate::__private::HasIterator = has_iter;
        while true {
            $crate::pounded_var_names!(quote_bind_next_or_break!() () $($inner)*);
            if _i > 0 {
                $crate::quote_token!($tokens $sep);
            }
            _i += 1;
            $crate::quote_each_token!($tokens $($inner)*);
        }
    }};
    ($tokens:ident $b3:tt $b2:tt # (( $($inner:tt)* )) $sep:tt * $a3:tt) => {};
    ($tokens:ident $b3:tt # ( $($inner:tt)* ) ($sep:tt) * $a2:tt $a3:tt) => {};
    ($tokens:ident # ( $($inner:tt)* ) * (*) $a1:tt $a2:tt $a3:tt) => {
        // https://github.com/dtolnay/quote/issues/130
        $crate::quote_token!($tokens *);
    };
    ($tokens:ident # ( $($inner:tt)* ) $sep:tt (*) $a1:tt $a2:tt $a3:tt) => {};

    ($tokens:ident $b3:tt $b2:tt $b1:tt (#) $var:ident $a2:tt $a3:tt) => {
        $crate::ToTokens::to_tokens(&$var, &mut $tokens);
    };
    ($tokens:ident $b3:tt $b2:tt # ($var:ident) $a1:tt $a2:tt $a3:tt) => {};
    ($tokens:ident $b3:tt $b2:tt $b1:tt ($curr:tt) $a1:tt $a2:tt $a3:tt) => {
        $crate::quote_token!($tokens $curr);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! quote_token {
    ($tokens:ident ( $($inner:tt)* )) => {
        $crate::__private::push_group(
            &mut $tokens,
            $crate::Delimiter::Parenthesis,
            $crate::quote!($($inner)*),
        );
    };

    ($tokens:ident [ $($inner:tt)* ]) => {
        $crate::__private::push_group(
            &mut $tokens,
            $crate::Delimiter::Bracket,
            $crate::quote!($($inner)*),
        );
    };

    ($tokens:ident { $($inner:tt)* }) => {
        $crate::__private::push_group(
            &mut $tokens,
            $crate::Delimiter::Brace,
            $crate::quote!($($inner)*),
        );
    };

    ($tokens:ident +) => {
        $crate::__private::push_add(&mut $tokens);
    };

    ($tokens:ident +=) => {
        $crate::__private::push_add_eq(&mut $tokens);
    };

    ($tokens:ident &) => {
        $crate::__private::push_and(&mut $tokens);
    };

    ($tokens:ident &&) => {
        $crate::__private::push_and_and(&mut $tokens);
    };

    ($tokens:ident &=) => {
        $crate::__private::push_and_eq(&mut $tokens);
    };

    ($tokens:ident @) => {
        $crate::__private::push_at(&mut $tokens);
    };

    ($tokens:ident !) => {
        $crate::__private::push_bang(&mut $tokens);
    };

    ($tokens:ident ^) => {
        $crate::__private::push_caret(&mut $tokens);
    };

    ($tokens:ident ^=) => {
        $crate::__private::push_caret_eq(&mut $tokens);
    };

    ($tokens:ident :) => {
        $crate::__private::push_colon(&mut $tokens);
    };

    ($tokens:ident ::) => {
        $crate::__private::push_colon2(&mut $tokens);
    };

    ($tokens:ident ,) => {
        $crate::__private::push_comma(&mut $tokens);
    };

    ($tokens:ident /) => {
        $crate::__private::push_div(&mut $tokens);
    };

    ($tokens:ident /=) => {
        $crate::__private::push_div_eq(&mut $tokens);
    };

    ($tokens:ident .) => {
        $crate::__private::push_dot(&mut $tokens);
    };

    ($tokens:ident ..) => {
        $crate::__private::push_dot2(&mut $tokens);
    };

    ($tokens:ident ...) => {
        $crate::__private::push_dot3(&mut $tokens);
    };

    ($tokens:ident ..=) => {
        $crate::__private::push_dot_dot_eq(&mut $tokens);
    };

    ($tokens:ident =) => {
        $crate::__private::push_eq(&mut $tokens);
    };

    ($tokens:ident ==) => {
        $crate::__private::push_eq_eq(&mut $tokens);
    };

    ($tokens:ident >=) => {
        $crate::__private::push_ge(&mut $tokens);
    };

    ($tokens:ident >) => {
        $crate::__private::push_gt(&mut $tokens);
    };

    ($tokens:ident <=) => {
        $crate::__private::push_le(&mut $tokens);
    };

    ($tokens:ident <) => {
        $crate::__private::push_lt(&mut $tokens);
    };

    ($tokens:ident *=) => {
        $crate::__private::push_mul_eq(&mut $tokens);
    };

    ($tokens:ident !=) => {
        $crate::__private::push_ne(&mut $tokens);
    };

    ($tokens:ident |) => {
        $crate::__private::push_or(&mut $tokens);
    };

    ($tokens:ident |=) => {
        $crate::__private::push_or_eq(&mut $tokens);
    };

    ($tokens:ident ||) => {
        $crate::__private::push_or_or(&mut $tokens);
    };

    ($tokens:ident #) => {
        $crate::__private::push_pound(&mut $tokens);
    };

    ($tokens:ident ?) => {
        $crate::__private::push_question(&mut $tokens);
    };

    ($tokens:ident ->) => {
        $crate::__private::push_rarrow(&mut $tokens);
    };

    ($tokens:ident <-) => {
        $crate::__private::push_larrow(&mut $tokens);
    };

    ($tokens:ident %) => {
        $crate::__private::push_rem(&mut $tokens);
    };

    ($tokens:ident %=) => {
        $crate::__private::push_rem_eq(&mut $tokens);
    };

    ($tokens:ident =>) => {
        $crate::__private::push_fat_arrow(&mut $tokens);
    };

    ($tokens:ident ;) => {
        $crate::__private::push_semi(&mut $tokens);
    };

    ($tokens:ident <<) => {
        $crate::__private::push_shl(&mut $tokens);
    };

    ($tokens:ident <<=) => {
        $crate::__private::push_shl_eq(&mut $tokens);
    };

    ($tokens:ident >>) => {
        $crate::__private::push_shr(&mut $tokens);
    };

    ($tokens:ident >>=) => {
        $crate::__private::push_shr_eq(&mut $tokens);
    };

    ($tokens:ident *) => {
        $crate::__private::push_star(&mut $tokens);
    };

    ($tokens:ident -) => {
        $crate::__private::push_sub(&mut $tokens);
    };

    ($tokens:ident -=) => {
        $crate::__private::push_sub_eq(&mut $tokens);
    };

    ($tokens:ident $ident:ident) => {
        $crate::__private::push_ident(&mut $tokens, stringify!($ident));
    };

    ($tokens:ident $other:tt) => {
        $crate::__private::parse(&mut $tokens, stringify!($other));
    };
}
