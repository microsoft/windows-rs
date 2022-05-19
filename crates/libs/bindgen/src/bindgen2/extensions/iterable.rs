use super::*;

pub fn gen() -> TokenStream {
    quote! {
        impl<T: ::windows::core::RuntimeType> ::core::iter::IntoIterator for IIterable<T> {
            type Item = T;
            type IntoIter = IIterator<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                ::core::iter::IntoIterator::into_iter(&self)
            }
        }
        impl<T: ::windows::core::RuntimeType> ::core::iter::IntoIterator for &IIterable<T> {
            type Item = T;
            type IntoIter = IIterator<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                // TODO: not sure how to avoid this unwrap, although it should always succeed in practice.
                self.First().unwrap()
            }
        }
    }
}
