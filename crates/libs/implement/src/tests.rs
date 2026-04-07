//! Tests for the `#[implement]` macro that verify the generated code structure.
//!
//! These tests call `implement_core` directly and check that the formatted output contains
//! the expected declarations and trait implementations.  Any change to the code generator
//! that silently removes or renames a key item will be caught as a test failure.
//!
//! To inspect the full formatted output of a test, run with `--nocapture`:
//!
//! ```text
//! cargo test -p windows-implement --lib -- --nocapture --test-threads=1
//! ```

use std::io::{Read, Write};
use std::process::{Command, Stdio};

use proc_macro2::TokenStream;
use quote::quote;

fn implement(attributes: TokenStream, item_tokens: TokenStream) -> String {
    let out_tokens = crate::implement_core(attributes, item_tokens);
    let tokens_string = out_tokens.to_string();

    let out_string = rustfmt(&tokens_string);
    println!("// output of #[implement] :");
    println!();
    println!("{out_string}");
    out_string
}

fn check(output: &str, expected_items: &[&str]) {
    for item in expected_items {
        assert!(
            output.contains(item),
            "output does not contain expected item:\n  expected: {item}\n\nfull output:\n{output}"
        );
    }
}

fn rustfmt(input: &str) -> String {
    let mut rustfmt = Command::new("rustfmt");

    rustfmt.stdin(Stdio::piped());
    rustfmt.stdout(Stdio::piped());
    rustfmt.stderr(Stdio::inherit());

    let mut child = match rustfmt.spawn() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("failed to spawn rustfmt: {e:?}");
            return input.to_string();
        }
    };

    let mut stdout = child.stdout.take().unwrap();

    // spawn thread to read stdout
    let stdout_thread = std::thread::spawn(move || {
        let mut buf = String::new();
        stdout.read_to_string(&mut buf).unwrap();
        buf
    });

    // write unformatted into stdin
    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(input.as_bytes()).unwrap();
    drop(stdin);

    let stdout_string: String = stdout_thread.join().unwrap();

    let exit = child.wait().unwrap();
    if !exit.success() {
        eprintln!("rustfmt terminated with failure status code");
        return input.to_string();
    }

    stdout_string
}

#[test]
fn simple_type() {
    let output = implement(
        quote!(IFoo),
        quote! {
            struct Foo {
                x: u32,
            }
        },
    );
    check(
        &output,
        &[
            "struct Foo_Impl",
            "impl ::core::ops::Deref for Foo_Impl",
            "impl ::windows_core::IUnknownImpl for Foo_Impl",
            "impl ::windows_core::ComObjectInner for Foo",
            "impl ::core::convert::From<Foo> for ::windows_core::IUnknown",
            "impl ::core::convert::From<Foo> for ::windows_core::IInspectable",
            "impl ::core::convert::From<Foo> for IFoo",
            "impl ::windows_core::ComObjectInterface<IFoo> for Foo_Impl",
            "impl ::windows_core::AsImpl<Foo> for IFoo",
        ],
    );
}

#[test]
fn zero_sized_type() {
    let output = implement(
        quote!(IFoo),
        quote! {
            struct Foo;
        },
    );
    check(
        &output,
        &[
            "struct Foo_Impl",
            "impl ::windows_core::IUnknownImpl for Foo_Impl",
            "impl ::windows_core::ComObjectInner for Foo",
            "impl ::core::convert::From<Foo> for IFoo",
            "impl ::windows_core::AsImpl<Foo> for IFoo",
        ],
    );
}

#[test]
fn no_interfaces() {
    let output = implement(
        quote!(),
        quote! {
            struct Foo {}
        },
    );
    check(
        &output,
        &[
            "struct Foo_Impl",
            "impl ::windows_core::IUnknownImpl for Foo_Impl",
            "impl ::windows_core::ComObjectInner for Foo",
            "impl ::core::convert::From<Foo> for ::windows_core::IUnknown",
        ],
    );
    // No interface-specific items should be present.
    assert!(
        !output.contains("impl ::core::convert::From<Foo> for IFoo"),
        "no_interfaces output unexpectedly contains From<Foo> for IFoo"
    );
    assert!(
        !output.contains("impl ::windows_core::AsImpl<Foo>"),
        "no_interfaces output unexpectedly contains AsImpl<Foo>"
    );
}

#[test]
fn generic_no_lifetime() {
    let output = implement(
        quote!(IAsyncOperationWithProgress<T, P>, IAsyncInfo),
        quote! {
            struct OperationWithProgress<T: RuntimeType + 'static, P>(SyncState<IAsyncOperationWithProgress<T, P>>)
            where
                P: RuntimeType + 'static;

        },
    );
    check(
        &output,
        &[
            "struct OperationWithProgress_Impl",
            "impl",
            "::windows_core::IUnknownImpl for OperationWithProgress_Impl",
            "impl",
            "::windows_core::ComObjectInner for OperationWithProgress",
            "impl",
            "::core::convert::From<OperationWithProgress",
            "impl",
            "::windows_core::AsImpl<OperationWithProgress",
            "for IAsyncInfo",
        ],
    );
    // Generic types cannot have into_static.
    assert!(
        !output.contains("into_static"),
        "generic_no_lifetime output unexpectedly contains into_static"
    );
}

#[test]
fn generic_with_lifetime() {
    let output = implement(
        quote!(),
        quote! {
            pub struct Foo<'a> {
                pub x: &'a [u8],
            }
        },
    );
    check(
        &output,
        &[
            "struct Foo_Impl",
            "impl",
            "::windows_core::IUnknownImpl for Foo_Impl",
            "impl",
            "::windows_core::ComObjectInner for Foo",
        ],
    );
    // Lifetime generics should also suppress into_static.
    assert!(
        !output.contains("into_static"),
        "generic_with_lifetime output unexpectedly contains into_static"
    );
}

#[test]
fn tuple_type() {
    let output = implement(
        quote!(IFoo),
        quote! {
            struct Foo(pub i32);
        },
    );
    check(
        &output,
        &[
            "struct Foo_Impl",
            "impl ::windows_core::IUnknownImpl for Foo_Impl",
            "impl ::windows_core::ComObjectInner for Foo",
            "impl ::core::convert::From<Foo> for IFoo",
            "impl ::windows_core::AsImpl<Foo> for IFoo",
        ],
    );
}

#[test]
fn two_interfaces() {
    let output = implement(
        quote!(IFoo, IBar),
        quote! {
            struct Baz;
        },
    );
    check(
        &output,
        &[
            "struct Baz_Impl",
            "impl ::windows_core::IUnknownImpl for Baz_Impl",
            "impl ::core::convert::From<Baz> for IFoo",
            "impl ::core::convert::From<Baz> for IBar",
            "impl ::windows_core::ComObjectInterface<IFoo> for Baz_Impl",
            "impl ::windows_core::ComObjectInterface<IBar> for Baz_Impl",
            "impl ::windows_core::AsImpl<Baz> for IFoo",
            "impl ::windows_core::AsImpl<Baz> for IBar",
        ],
    );
}

#[test]
fn not_agile() {
    let output = implement(
        quote!(IFoo, Agile = false),
        quote! {
            struct Foo;
        },
    );
    // When Agile=false, IAgileObject should NOT appear in QueryInterface.
    assert!(
        !output.contains("IAgileObject"),
        "not_agile output unexpectedly contains IAgileObject"
    );
}

#[test]
fn namespaced_interface() {
    let output = implement(
        quote!(Windows::Win32::IFoo),
        quote! {
            struct Foo;
        },
    );
    check(
        &output,
        &[
            "struct Foo_Impl",
            "impl ::windows_core::AsImpl<Foo> for Windows::Win32::IFoo",
        ],
    );
}
