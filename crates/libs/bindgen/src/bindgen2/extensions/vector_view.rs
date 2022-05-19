use super::*;

pub fn gen() -> TokenStream {
    quote! {
        pub struct VectorViewIterator<T: ::windows::core::RuntimeType + 'static> {
            vector: ::core::option::Option<IVectorView<T>>,
            current: u32,
        }

        impl<T: ::windows::core::RuntimeType> VectorViewIterator<T> {
            pub fn new(vector: ::core::option::Option<IVectorView<T>>) -> Self {
                Self { vector, current: 0 }
            }
        }

        impl<T: ::windows::core::RuntimeType> ::core::iter::Iterator for VectorViewIterator<T> {
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

        impl<T: ::windows::core::RuntimeType> ::core::iter::IntoIterator for IVectorView<T> {
            type Item = T;
            type IntoIter = VectorViewIterator<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                ::core::iter::IntoIterator::into_iter(&self)
            }
        }
        impl<T: ::windows::core::RuntimeType> ::core::iter::IntoIterator for &IVectorView<T> {
            type Item = T;
            type IntoIter = VectorViewIterator<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                // TODO: shouldn't need to clone - VectorViewIterator should hold a reference
                VectorViewIterator::new(::core::option::Option::Some(::core::clone::Clone::clone(self)))
            }
        }
    }
}
