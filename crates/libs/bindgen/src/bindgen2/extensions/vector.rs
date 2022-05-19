use super::*;

pub fn gen() -> TokenStream {
    quote! {
        pub struct VectorIterator<T: ::windows::core::RuntimeType + 'static> {
            vector: ::core::option::Option<IVector<T>>,
            current: u32,
        }

        impl<T: ::windows::core::RuntimeType> VectorIterator<T> {
            pub fn new(vector: ::core::option::Option<IVector<T>>) -> Self {
                Self { vector, current: 0 }
            }
        }

        impl<T: ::windows::core::RuntimeType> ::core::iter::Iterator for VectorIterator<T> {
            type Item = T;

            fn next(&mut self) -> ::core::option::Option<Self::Item> {
                self.vector.as_ref()
                    .and_then(|vector| {
                        vector.GetAt(self.current).ok()
                    })
                    .and_then(|result| {
                        self.current += 1;
                        Some(result)
                    })
            }
        }

        impl<T: ::windows::core::RuntimeType> ::core::iter::IntoIterator for IVector<T> {
            type Item = T;
            type IntoIter = VectorIterator<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                ::core::iter::IntoIterator::into_iter(&self)
            }
        }
        impl<T: ::windows::core::RuntimeType> ::core::iter::IntoIterator for &IVector<T> {
            type Item = T;
            type IntoIter = VectorIterator<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                // TODO: shouldn't need to clone - VectorIterator should hold a reference
                VectorIterator::new(::core::option::Option::Some(::core::clone::Clone::clone(self)))
            }
        }
    }
}
