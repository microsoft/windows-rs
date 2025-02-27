use std::io::{Read, Write};
use std::process::{Command, Stdio};

use proc_macro2::TokenStream;
use quote::quote;

fn implement(attributes: TokenStream, item_tokens: TokenStream) -> String {
    let out_tokens = crate::implement_core(attributes, item_tokens);
    let tokens_string = out_tokens.to_string();

    let out_string = rustfmt(&tokens_string);
    println!("output of #[implement] :");
    println!();
    println!("{}", out_string);
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

#[test]
fn base() {
    let _out = implement(
        quote!(IBase, IOtherBase),
        quote! {
            struct Base {
                zzz: u32,
            }
        },
    );
}

#[test]
fn derived() {
    let _out = implement(
        quote!(IFoo, IBar, IZap),
        quote! {
            struct Derived {
                #[base]
                base: Base_Impl,

                x: u32,
            }
        },
    );
}

#[test]
fn generic_no_lifetime() {
    let _out = implement(
        quote!(IAsyncOperationWithProgress<T, P>, IAsyncInfo),
        quote! {
            struct OperationWithProgress<T, P>(SyncState<IAsyncOperationWithProgress<T, P>>)
            where
                T: RuntimeType + 'static,
                P: RuntimeType + 'static;

        },
    );
}

#[test]
fn generic_with_lifetime() {
    let _out = implement(
        quote!(),
        quote! {
            pub struct Foo<'a> {
                pub x: &'a [u8],
            }

        },
    );
}
