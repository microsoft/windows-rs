use crate::tables::*;
use crate::types::*;
use crate::*;
use proc_macro2::TokenStream;
use quote::quote;
use std::iter::FromIterator;

pub fn iterator_tokens(name: &TypeName, interfaces: &Vec<RequiredInterface>) -> TokenStream {
    if name.name == "IIterator`1"
        && name.namespace == "Windows.Foundation.Collections"
    {
        return quote! {
            impl<T: ::winrt::RuntimeType> ::std::iter::Iterator for IIterator<T> {
                type Item = T;

                fn next(&mut self) -> ::std::option::Option<Self::Item> {
                    let result = self.current().ok();

                    if result.is_some() {
                        self.move_next().ok()?;
                    }

                    result
                }
            }
        };
    }

    if name.name == "IVectorView`1"
        && name.namespace == "Windows.Foundation.Collections"
    {
        return quote! {
            pub struct VectorViewIterator<T: ::winrt::RuntimeType + 'static> {
                vector: IVectorView<T>,
                current: u32,
                size: u32,
            }

            impl<T: ::winrt::RuntimeType> VectorViewIterator<T> {
                pub fn new(vector: IVectorView<T>) -> Self {
                    let size = vector.size().unwrap();
                    Self { vector, current: 0, size }
                }
            }

            impl<T: ::winrt::RuntimeType> Iterator for VectorViewIterator<T> {
                type Item = T;

                fn next(&mut self) -> Option<Self::Item> {
                    if self.current >= self.size {
                        return None;
                    }

                    let result = self.vector.get_at(self.current);
                    self.current += 1;
                    result.ok()
                }
            }

            impl<T: ::winrt::RuntimeType> IntoIterator for &IVectorView<T> {
                type Item = T;
                type IntoIter = VectorViewIterator<Self::Item>;
            
                fn into_iter(self) -> Self::IntoIter {
                    VectorViewIterator::new(self.clone())
                }
            }
        };
    }

    if name.name == "IVector`1" && name.namespace == "Windows.Foundation.Collections"
    {
        return quote! {
            pub struct VectorIterator<T: ::winrt::RuntimeType + 'static> {
                vector: IVector<T>,
                current: u32,
                size: u32,
            }

            impl<T: ::winrt::RuntimeType> VectorIterator<T> {
                pub fn new(vector: IVector<T>) -> Self {
                    let size = vector.size().unwrap();
                    Self { vector, current: 0, size }
                }
            }

            impl<T: ::winrt::RuntimeType> Iterator for VectorIterator<T> {
                type Item = T;

                fn next(&mut self) -> Option<Self::Item> {
                    if self.current >= self.size {
                        return None;
                    }

                    let result = self.vector.get_at(self.current);
                    self.current += 1;
                    result.ok()
                }
            }

            impl<T: ::winrt::RuntimeType> IntoIterator for &IVector<T> {
                type Item = T;
                type IntoIter = VectorIterator<Self::Item>;
            
                fn into_iter(self) -> Self::IntoIter {
                    VectorIterator::new(self.clone())
                }
            }
        };
    }

    // TODO: do this for both classes and interfaces:

    // TODO: next, search if any required interface is IVector, IVectorView, or Iteratable (in that order)
    // and add an IntoItertor impl if that's the case

    TokenStream::new()
}