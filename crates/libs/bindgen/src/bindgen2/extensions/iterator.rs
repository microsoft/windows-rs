use super::*;

pub fn gen() -> TokenStream {
    quote! {
        impl<T: ::windows::core::RuntimeType> ::core::iter::Iterator for IIterator<T> {
            type Item = T;
            fn next(&mut self) -> ::core::option::Option<Self::Item> {
                let result = self.Current().ok();
                if result.is_some() {
                    self.MoveNext().ok()?;
                }
                result
            }
        }
    }
}
