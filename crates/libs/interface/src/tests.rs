//! Tests for the `#[interface]` macro that verify the generated code structure.
//!
//! These tests call `interface_core` directly and check that the formatted output contains
//! the expected declarations, vtable, and trait definitions.  Any change to the code generator
//! that silently removes or renames a key item will be caught as a test failure.
//!
//! To inspect the full formatted output of a test, run with `--nocapture`:
//!
//! ```text
//! cargo test -p windows-interface --lib -- --nocapture --test-threads=1
//! ```

use std::io::{Read, Write};
use std::process::{Command, Stdio};

use proc_macro2::TokenStream;
use quote::quote;

fn interface(attributes: TokenStream, item_tokens: TokenStream) -> String {
    let out_tokens = crate::interface_core(attributes, item_tokens);
    let tokens_string = out_tokens.to_string();

    let out_string = rustfmt(&tokens_string);
    println!("// output of #[interface] :");
    println!();
    println!("{out_string}");
    out_string
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

fn check(output: &str, expected_items: &[&str]) {
    for item in expected_items {
        assert!(
            output.contains(item),
            "output does not contain expected item:\n  expected: {item}\n\nfull output:\n{output}"
        );
    }
}

#[test]
fn com_interface_with_parent() {
    let output = interface(
        quote!("094d70d6-5202-44b8-abb8-43860da5aca2"),
        quote! {
            unsafe trait IFoo: IUnknown {
                fn GetValue(&self, value: *mut i32) -> HRESULT;
            }
        },
    );
    check(
        &output,
        &[
            "struct IFoo",
            "unsafe impl ::windows_core::Interface for IFoo",
            "impl ::windows_core::RuntimeName for IFoo",
            "impl ::core::ops::Deref for IFoo",
            "trait IFoo_Impl",
            "unsafe fn GetValue",
            "struct IFoo_Vtbl",
            "pub GetValue:",
            "impl IFoo_Vtbl",
            "pub const fn new<",
            "pub fn matches",
            "impl ::core::convert::From<IFoo> for ::windows_core::IUnknown",
            "impl ::core::clone::Clone for IFoo",
            "impl ::core::fmt::Debug for IFoo",
        ],
    );
    // The IID should contain the parsed GUID values.
    assert!(
        output.contains("0x094d70d6"),
        "output does not contain expected GUID data1"
    );
}

#[test]
fn com_interface_inheriting_parent() {
    let output = interface(
        quote!("00000000-0000-0000-0000-000000000002"),
        quote! {
            unsafe trait IBar: IFoo {
                fn SetValue(&self, value: i32) -> HRESULT;
            }
        },
    );
    check(
        &output,
        &[
            "struct IBar",
            "unsafe impl ::windows_core::Interface for IBar",
            "impl ::core::ops::Deref for IBar",
            // IBar_Impl must require IFoo_Impl as a supertrait.
            "trait IBar_Impl",
            "IFoo_Impl",
            "struct IBar_Vtbl",
            // base__ comes from parent vtable.
            "pub base__:",
            "pub SetValue:",
            "pub fn matches",
            // matches must chain to parent.
            "|| <",
            ">::matches(iid)",
        ],
    );
}

#[test]
fn non_com_interface() {
    // Non-COM interface: no parent (no IUnknown base).
    let output = interface(
        quote!(),
        quote! {
            unsafe trait ICallback {
                fn Invoke(&self, result: i32);
            }
        },
    );
    check(
        &output,
        &[
            "struct ICallback",
            "unsafe impl ::windows_core::Interface for ICallback",
            "trait ICallback_Impl",
            "unsafe fn Invoke",
            "struct ICallback_Vtbl",
            "pub Invoke:",
            // Non-COM path uses simpler Impl-generic constructor.
            "pub const fn new<Impl: ICallback_Impl>",
            // Non-COM interfaces also generate the ImplVtbl helper.
            "struct ICallback_ImplVtbl",
            // And a Foo::new constructor for ScopedInterface.
            "fn new<'a, T: ICallback_Impl>",
        ],
    );
    // Non-COM interface should NOT emit a `matches` function (no QueryInterface).
    assert!(
        !output.contains("pub fn matches"),
        "non_com_interface output unexpectedly contains `matches`"
    );
}

#[test]
fn interface_no_guid() {
    // Omitting the GUID should produce GUID::zeroed().
    let output = interface(
        quote!(),
        quote! {
            unsafe trait INoGuid: IUnknown {}
        },
    );
    check(
        &output,
        &[
            "struct INoGuid",
            "::windows_core::GUID::zeroed()",
            "trait INoGuid_Impl",
        ],
    );
}

#[test]
fn interface_with_result_return() {
    // Methods returning `Result<T>` should get the `.ok()` wrapper and HRESULT vtable entry.
    let output = interface(
        quote!("00000000-0000-0000-0000-000000000003"),
        quote! {
            unsafe trait IReader: IUnknown {
                fn Read(&self, buf: *mut u8, len: u32) -> Result<u32>;
            }
        },
    );
    check(
        &output,
        &[
            "fn Read",
            // Caller side wraps with .ok()
            ".ok()",
            // Vtable entry must use HRESULT.
            "-> ::windows_core::HRESULT",
        ],
    );
}

#[test]
fn public_visibility() {
    let output = interface(
        quote!("00000000-0000-0000-0000-000000000004"),
        quote! {
            pub unsafe trait IPublic: IUnknown {}
        },
    );
    // Visibility must be propagated to the struct and trait.
    check(&output, &["pub struct IPublic", "pub trait IPublic_Impl"]);
}
