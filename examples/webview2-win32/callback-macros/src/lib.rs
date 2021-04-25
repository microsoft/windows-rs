use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Ident, Result, Token, TypePath, Visibility,
};

struct CallbackTypes {
    pub interface: TypePath,
    pub arg_1: TypePath,
    pub arg_2: TypePath,
}

impl Parse for CallbackTypes {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        parenthesized!(content in input);
        let args: Punctuated<TypePath, Token![,]> = content.parse_terminated(TypePath::parse)?;
        input.parse::<Token![;]>()?;
        if args.len() == 3 {
            let mut args = args.into_iter();

            Ok(CallbackTypes {
                interface: args.next().unwrap(),
                arg_1: args.next().unwrap(),
                arg_2: args.next().unwrap(),
            })
        } else {
            Err(content.error("expected (interface, arg_1, arg_2)"))
        }
    }
}

struct CallbackStruct {
    pub vis: Visibility,
    pub struct_token: Token![struct],
    pub ident: Ident,
    pub args: CallbackTypes,
}

impl Parse for CallbackStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(CallbackStruct {
            vis: input.parse()?,
            struct_token: input.parse()?,
            ident: input.parse()?,
            args: input.parse()?,
        })
    }
}

/// Implement a `CompletedCallback` using the types specified as tuple struct fields.
#[proc_macro_attribute]
pub fn completed_callback(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as CallbackStruct);
    impl_completed_callback(&ast)
}

fn impl_completed_callback(ast: &CallbackStruct) -> TokenStream {
    let vis = &ast.vis;

    let name = &ast.ident;
    let closure = get_closure(name);

    let interface = &ast.args.interface;
    let abi = get_abi(interface);

    let arg_1 = &ast.args.arg_1;
    let arg_2 = &ast.args.arg_2;

    let gen = quote! {
        use windows as _;

        type #closure<'a> = CompletedClosure<'a, #arg_1, #arg_2>;

        /// Implementation of [`#interface`].
        #[repr(C)]
        #vis struct #name<'a> {
            vtable: *const #abi,
            count: windows::RefCount,
            completed: Option<#closure<'a>>,
        }

        impl<'a> #name<'a> {
            /// Factory method which returns a [`windows::Result<#interface>`] wrapped around a new instance of [`#name`]
            pub fn create(completed: #closure<'a>) -> windows::Result<#interface> {
                let handler = Box::new(Self::new(completed));
                let handler = unsafe { Self::from_abi(Box::into_raw(handler) as windows::RawPtr)? };
                Ok(handler)
            }

            unsafe fn from_abi(this: windows::RawPtr) -> windows::Result<#interface> {
                let unknown = windows::IUnknown::from_abi(this)?;
                unknown.vtable().1(unknown.abi()); // add_ref to balance the release called in IUnknown::drop
                unknown.cast()
            }

            fn new(completed: #closure<'a>) -> Self {
                static VTABLE: #abi = #abi(
                    #name::QueryInterface,
                    #name::AddRef,
                    #name::Release,
                    #name::Invoke,
                );

                Self {
                   vtable: &VTABLE,
                   count: windows::RefCount::new(),
                   completed: Some(completed),
                }
            }

            pub fn wait_for_async_operation(
                closure: Box<dyn FnOnce(<Self as Callback<'a>>::Interface) -> windows::HRESULT>,
                completed: <Self as Callback<'a>>::Closure,
            ) -> super::Result<()> {
                let (tx, rx) = mpsc::channel();
                let completed = Box::new(move |arg_1, arg_2| {
                    tx.send(completed(arg_1, arg_2))
                        .expect("send over mpsc channel");
                    S_OK
                });
                let callback = Self::create(completed)?;

                let mut error_code = closure(callback);
                if error_code.is_ok() {
                  error_code = wait_with_pump(rx)?;
                }

                if error_code.is_err() {
                  return Err(windows::Error::fast_error(error_code).into());
                }

                Ok(())
            }
        }

        impl<'a> Callback<'a> for #name<'a> {
            type Interface = #interface;
            type Closure = #closure<'a>;
        }

        impl<'a> CallbackInterface<'a, #name<'a>> for #name<'a> {
            fn ref_count(&self) -> &windows::RefCount {
                &self.count
            }
        }

        impl<'a> CompletedCallback<'a, #name<'a>, #arg_1, #arg_2> for #name<'a> {
            fn completed(&mut self) -> Option<#closure<'a>> {
                self.completed.take()
            }
        }
    };

    gen.into()
}

/// Implement an `EventCallback` using the types specified as tuple struct fields.
#[proc_macro_attribute]
pub fn event_callback(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as CallbackStruct);
    impl_event_callback(&ast)
}

fn impl_event_callback(ast: &CallbackStruct) -> TokenStream {
    let vis = &ast.vis;

    let name = &ast.ident;
    let closure = get_closure(name);

    let interface = &ast.args.interface;
    let abi = get_abi(interface);

    let arg_1 = &ast.args.arg_1;
    let arg_2 = &ast.args.arg_2;

    let gen = quote! {
        type #closure<'a> = EventClosure<'a, #arg_1, #arg_2>;

        /// Implementation of [`#interface`].
        #[repr(C)]
        #vis struct #name<'a> {
            vtable: *const #abi,
            count: windows::RefCount,
            event: #closure<'a>,
        }

        impl<'a> #name<'a> {
            /// Factory method which returns a [`windows::Result<#interface>`] wrapped around a new instance of [`#name`].
            pub fn create(event: #closure<'a>) -> windows::Result<#interface> {
                let handler = Box::new(Self::new(event));
                let handler = unsafe { Self::from_abi(Box::into_raw(handler) as windows::RawPtr)? };
                Ok(handler)
            }

            unsafe fn from_abi(this: windows::RawPtr) -> windows::Result<#interface> {
                let unknown = windows::IUnknown::from_abi(this)?;
                unknown.vtable().1(unknown.abi()); // add_ref to balance the release called in IUnknown::drop
                unknown.cast()
            }

            fn new(event: #closure<'a>) -> Self {
                static VTABLE: #abi = #abi(
                    #name::QueryInterface,
                    #name::AddRef,
                    #name::Release,
                    #name::Invoke,
                );

                Self {
                    vtable: &VTABLE,
                    count: windows::RefCount::new(),
                    event,
                }
            }
        }

        impl<'a> Callback<'a> for #name<'a> {
            type Interface = #interface;
            type Closure = #closure<'a>;
        }

        impl<'a> CallbackInterface<'a, #name<'a>> for #name<'a> {
            fn ref_count(&self) -> &windows::RefCount {
                &self.count
            }
        }

        impl<'a> EventCallback<'a, #name<'a>, #arg_1, #arg_2> for #name<'a> {
            fn event(&mut self) -> &mut #closure<'a> {
                &mut self.event
            }
        }
    };

    gen.into()
}

fn get_closure(name: &Ident) -> Ident {
    format_ident!("{}Closure", name)
}

fn get_abi(interface: &TypePath) -> TypePath {
    let mut abi = interface.clone();
    let last_ident = &mut abi
        .path
        .segments
        .last_mut()
        .expect("abi.path.segments.last_mut()")
        .ident;
    *last_ident = format_ident!("{}_abi", last_ident);

    abi
}
