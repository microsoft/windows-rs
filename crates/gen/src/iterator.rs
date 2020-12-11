use crate::*;
use squote::{quote, TokenStream};

// Provides iterator support for the well-known WinRT collection interfaces and any classes or
// interfaces that implement any of these interfaces. It also favors high-speed iteration and
// only falls back to IIterator<T> if nothing faster is available. VectorIterator and
// VectorViewIterator are faster iterators than IIterator<T> because they only require a single
// vcall per iteration wheras IIterator<T> requires two.
pub fn gen_iterator(name: &TypeName, interfaces: &[RequiredInterface]) -> TokenStream {
    // If the type is IIterator<T> then simply implement the Iterator trait over top.
    if name.name == "IIterator`1" && name.namespace == "Windows.Foundation.Collections" {
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

    // If the type is IIterable<T> then implement the IntoIterator trait and rely on the resulting
    // IIterator<T> returned by first() to implement the Iterator trait.
    if name.name == "IIterable`1" && name.namespace == "Windows.Foundation.Collections" {
        return quote! {
            impl<T: ::winrt::RuntimeType> ::std::iter::IntoIterator for IIterable<T> {
                type Item = T;
                type IntoIter = IIterator<Self::Item>;

                fn into_iter(self) -> Self::IntoIter {
                    self.first().unwrap()
                }
            }
            impl<'a, T: ::winrt::RuntimeType> ::std::iter::IntoIterator for &'a IIterable<T> {
                type Item = T;
                type IntoIter = IIterator<Self::Item>;

                fn into_iter(self) -> Self::IntoIter {
                    self.first().unwrap()
                }
            }
        };
    }

    // If the type is IVectorView<T> then provide the VectorViewIterator fast iterator.
    if name.name == "IVectorView`1" && name.namespace == "Windows.Foundation.Collections" {
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

            impl<T: ::winrt::RuntimeType> ::std::iter::Iterator for VectorViewIterator<T> {
                type Item = T;

                fn next(&mut self) -> ::std::option::Option<Self::Item> {
                    if self.current >= self.size {
                        return None;
                    }

                    let result = self.vector.get_at(self.current);
                    self.current += 1;
                    result.ok()
                }
            }

            impl<T: ::winrt::RuntimeType> ::std::iter::IntoIterator for IVectorView<T> {
                type Item = T;
                type IntoIter = VectorViewIterator<Self::Item>;

                fn into_iter(self) -> Self::IntoIter {
                    VectorViewIterator::new(self)
                }
            }
            impl<'a, T: ::winrt::RuntimeType> ::std::iter::IntoIterator for &'a IVectorView<T> {
                type Item = T;
                type IntoIter = VectorViewIterator<Self::Item>;

                fn into_iter(self) -> Self::IntoIter {
                    VectorViewIterator::new(::std::clone::Clone::clone(self))
                }
            }
        };
    }

    // If the type is IVector<T> then provide the VectorIterator fast iterator.
    if name.name == "IVector`1" && name.namespace == "Windows.Foundation.Collections" {
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

            impl<T: ::winrt::RuntimeType> ::std::iter::Iterator for VectorIterator<T> {
                type Item = T;

                fn next(&mut self) -> ::std::option::Option<Self::Item> {
                    if self.current >= self.size {
                        return None;
                    }

                    let result = self.vector.get_at(self.current);
                    self.current += 1;
                    result.ok()
                }
            }

            impl<T: ::winrt::RuntimeType> ::std::iter::IntoIterator for IVector<T> {
                type Item = T;
                type IntoIter = VectorIterator<Self::Item>;

                fn into_iter(self) -> Self::IntoIter {
                    VectorIterator::new(self)
                }
            }
            impl<'a, T: ::winrt::RuntimeType> ::std::iter::IntoIterator for &'a IVector<T> {
                type Item = T;
                type IntoIter = VectorIterator<Self::Item>;

                fn into_iter(self) -> Self::IntoIter {
                    VectorIterator::new(::std::clone::Clone::clone(self))
                }
            }
        };
    }

    let mut iterable = None;

    // If the class or interface is not one of the well-known collection interfaces, we then see whether it
    // implements any one of them. Here is where we favor IVectorView/IVector over IIterable.
    for interface in interfaces {
        if interface.name.name == "IVectorView`1"
            && interface.name.namespace == "Windows.Foundation.Collections"
        {
            let item = interface.name.generics[0].gen();
            let wfc = gen_namespace(&interface.name.namespace, &name.namespace);
            let name = name.gen();

            return quote! {
                impl ::std::iter::IntoIterator for #name {
                    type Item = #item;
                    type IntoIter = #wfc VectorViewIterator<Self::Item>;

                    fn into_iter(self) -> Self::IntoIter {
                        #wfc VectorViewIterator::new(self.into())
                    }
                }
                impl<'a> ::std::iter::IntoIterator for &'a #name {
                    type Item = #item;
                    type IntoIter = #wfc VectorViewIterator<Self::Item>;

                    fn into_iter(self) -> Self::IntoIter {
                        #wfc VectorViewIterator::new(self.into())
                    }
                }
            };
        }

        if interface.name.name == "IVectorView`1"
            && interface.name.namespace == "Windows.Foundation.Collections"
        {
            let item = interface.name.generics[0].gen();
            let wfc = gen_namespace(&interface.name.namespace, &name.namespace);
            let name = name.gen();

            return quote! {
                impl ::std::iter::IntoIterator for #name {
                    type Item = #item;
                    type IntoIter = #wfc VectorIterator<Self::Item>;

                    fn into_iter(self) -> Self::IntoIter {
                        #wfc VectorIterator::new(self.into())
                    }
                }
                impl<'a> ::std::iter::IntoIterator for &'a #name {
                    type Item = #item;
                    type IntoIter = #wfc VectorIterator<Self::Item>;

                    fn into_iter(self) -> Self::IntoIter {
                        #wfc VectorIterator::new(self.into())
                    }
                }
            };
        }

        if interface.name.name == "IIterable`1"
            && interface.name.namespace == "Windows.Foundation.Collections"
        {
            iterable = Some(interface);
        }
    }

    match iterable {
        None => TokenStream::new(),
        Some(interface) => {
            let constraints = name.gen_constraint();
            let item = interface.name.generics[0].gen();
            let wfc = gen_namespace(&interface.name.namespace, &name.namespace);
            let name = name.gen();

            quote! {
               impl<#constraints> ::std::iter::IntoIterator for #name {
                    type Item = #item;
                    type IntoIter = #wfc IIterator<Self::Item>;

                    fn into_iter(self) -> Self::IntoIter {
                        self.first().unwrap()
                    }
                }
                impl<'a, #constraints> ::std::iter::IntoIterator for &'a #name {
                    type Item = #item;
                    type IntoIter = #wfc IIterator<Self::Item>;

                    fn into_iter(self) -> Self::IntoIter {
                        self.first().unwrap()
                    }
                }
            }
        }
    }
}
