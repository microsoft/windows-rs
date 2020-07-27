use proc_macro2::TokenStream;
use quote::quote;

pub fn get_object_tokens() -> TokenStream {
    quote! {
        #[repr(transparent)]
        #[derive(Default, Clone)]
        pub struct Object {
            ptr: ::winrt::ComPtr<Object>,
        }

        impl Object {
            pub fn type_name(&self) -> ::winrt::Result<::winrt::HString> {
                let this = <Self as ::winrt::AbiTransferable>::get_abi(self)
                    .expect("The `this` pointer was null when calling method");

                let mut string = ::winrt::HString::default();

                unsafe {
                    (this.vtable().inspectable_type_name)(this, <::winrt::HString as ::winrt::AbiTransferable>::set_abi(&mut string)).ok()?;
                }

                Ok(string)
            }
        }

        unsafe impl ::winrt::ComInterface for Object {
            type VTable = abi_IInspectable;

            fn iid() -> ::winrt::Guid {
                ::winrt::Guid::from_values(
                    0xAF86_E2E0,
                    0xB12D,
                    0x4C6A,
                    [0x9C, 0x5A, 0xD7, 0xAA, 0x65, 0x10, 0x1E, 0x90],
                )
            }
        }

        unsafe impl ::winrt::RuntimeType for Object {
            fn signature() -> String {
                "cinterface(IInspectable)".to_owned()
            }
        }

        unsafe impl ::winrt::AbiTransferable for Object {
            type Abi = ::winrt::RawComPtr<Object>;

            fn get_abi(&self) -> Self::Abi {
                self.ptr.get_abi()
            }

            fn set_abi(&mut self) -> *mut Self::Abi {
                self.ptr.set_abi()
            }
        }

        #[repr(C)]
        pub struct abi_IInspectable {
            iunknown: ::winrt::abi_IUnknown,

            pub inspectable_iids:
                unsafe extern "system" fn(::winrt::NonNullRawComPtr<Object>, *mut u32, *mut *mut ::winrt::Guid) -> ::winrt::ErrorCode,

            pub inspectable_type_name: unsafe extern "system" fn(
                ::winrt::NonNullRawComPtr<Object>,
                *mut <::winrt::HString as ::winrt::AbiTransferable>::Abi,
            ) -> ::winrt::ErrorCode,

            pub inspectable_trust_level:
                unsafe extern "system" fn(::winrt::NonNullRawComPtr<Object>, *mut i32) -> ::winrt::ErrorCode,
        }
    }
}
