use super::*;

// Provides iterator support for the well-known WinRT collection interfaces and any classes or
// interfaces that implement any of these interfaces. It also favors high-speed iteration and
// only falls back to IIterator<T> if nothing faster is available. VectorIterator and
// VectorViewIterator are faster iterators than IIterator<T> because they only require a single
// vcall per iteration wheras IIterator<T> requires two.
pub fn gen_iterator(def: &GenericType, interfaces: &[InterfaceInfo], gen: &Gen) -> TokenStream {
    let name = def.def.full_name();

    // If the type is IIterator<T> then simply implement the Iterator trait over top.
    if name == ("Windows.Foundation.Collections", "IIterator`1") {
        return quote! {
            impl<T: ::windows::RuntimeType> ::std::iter::Iterator for IIterator<T> {
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
    if name == ("Windows.Foundation.Collections", "IIterable`1") {
        return quote! {
            impl<T: ::windows::RuntimeType> ::std::iter::IntoIterator for IIterable<T> {
                type Item = T;
                type IntoIter = IIterator<Self::Item>;

                fn into_iter(self) -> Self::IntoIter {
                    self.first().unwrap()
                }
            }
            impl<'a, T: ::windows::RuntimeType> ::std::iter::IntoIterator for &'a IIterable<T> {
                type Item = T;
                type IntoIter = IIterator<Self::Item>;

                fn into_iter(self) -> Self::IntoIter {
                    self.first().unwrap()
                }
            }
        };
    }

    // If the type is IVectorView<T> then provide the VectorViewIterator fast iterator.
    if name == ("Windows.Foundation.Collections", "IVectorView`1") {
        return quote! {
            pub struct VectorViewIterator<T: ::windows::RuntimeType + 'static> {
                vector: IVectorView<T>,
                current: u32,
                size: u32,
            }

            impl<T: ::windows::RuntimeType> VectorViewIterator<T> {
                pub fn new(vector: IVectorView<T>) -> Self {
                    let size = vector.size().unwrap();
                    Self { vector, current: 0, size }
                }
            }

            impl<T: ::windows::RuntimeType> ::std::iter::Iterator for VectorViewIterator<T> {
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

            impl<T: ::windows::RuntimeType> ::std::iter::IntoIterator for IVectorView<T> {
                type Item = T;
                type IntoIter = VectorViewIterator<Self::Item>;

                fn into_iter(self) -> Self::IntoIter {
                    VectorViewIterator::new(self)
                }
            }
            impl<'a, T: ::windows::RuntimeType> ::std::iter::IntoIterator for &'a IVectorView<T> {
                type Item = T;
                type IntoIter = VectorViewIterator<Self::Item>;

                fn into_iter(self) -> Self::IntoIter {
                    VectorViewIterator::new(::std::clone::Clone::clone(self))
                }
            }
        };
    }

    // If the type is IVector<T> then provide the VectorIterator fast iterator.
    if name == ("Windows.Foundation.Collections", "IVector`1") {
        return quote! {
            pub struct VectorIterator<T: ::windows::RuntimeType + 'static> {
                vector: IVector<T>,
                current: u32,
                size: u32,
            }

            impl<T: ::windows::RuntimeType> VectorIterator<T> {
                pub fn new(vector: IVector<T>) -> Self {
                    let size = vector.size().unwrap();
                    Self { vector, current: 0, size }
                }
            }

            impl<T: ::windows::RuntimeType> ::std::iter::Iterator for VectorIterator<T> {
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

            impl<T: ::windows::RuntimeType> ::std::iter::IntoIterator for IVector<T> {
                type Item = T;
                type IntoIter = VectorIterator<Self::Item>;

                fn into_iter(self) -> Self::IntoIter {
                    VectorIterator::new(self)
                }
            }
            impl<'a, T: ::windows::RuntimeType> ::std::iter::IntoIterator for &'a IVector<T> {
                type Item = T;
                type IntoIter = VectorIterator<Self::Item>;

                fn into_iter(self) -> Self::IntoIter {
                    VectorIterator::new(::std::clone::Clone::clone(self))
                }
            }
        };
    }

    let mut iterable = None;
    let wfc = gen.namespace("Windows.Foundation.Collections");

    // If the class or interface is not one of the well-known collection interfaces, we then see whether it
    // implements any one of them. Here is where we favor IVectorView/IVector over IIterable.
    for interface in interfaces {
        let name = interface.def.def.full_name();

        if name == ("Windows.Foundation.Collections", "IVectorView`1") {
            let constraints = def.gen_constraints(gen);
            let item = interface.def.generics[0].gen_name(gen);
            let name = def.gen_name(gen);

            return quote! {
                impl<#constraints> ::std::iter::IntoIterator for #name {
                    type Item = #item;
                    type IntoIter = #wfc VectorViewIterator<Self::Item>;

                    fn into_iter(self) -> Self::IntoIter {
                        #wfc VectorViewIterator::new(self.into())
                    }
                }
                impl<'a, #constraints> ::std::iter::IntoIterator for &'a #name {
                    type Item = #item;
                    type IntoIter = #wfc VectorViewIterator<Self::Item>;

                    fn into_iter(self) -> Self::IntoIter {
                        #wfc VectorViewIterator::new(self.into())
                    }
                }
            };
        }

        if name == ("Windows.Foundation.Collections", "IVector`1") {
            let constraints = def.gen_constraints(gen);
            let item = interface.def.generics[0].gen_name(gen);
            let name = def.gen_name(gen);

            return quote! {
                impl<#constraints> ::std::iter::IntoIterator for #name {
                    type Item = #item;
                    type IntoIter = #wfc VectorIterator<Self::Item>;

                    fn into_iter(self) -> Self::IntoIter {
                        #wfc VectorIterator::new(self.into())
                    }
                }
                impl<'a, #constraints> ::std::iter::IntoIterator for &'a #name {
                    type Item = #item;
                    type IntoIter = #wfc VectorIterator<Self::Item>;

                    fn into_iter(self) -> Self::IntoIter {
                        #wfc VectorIterator::new(self.into())
                    }
                }
            };
        }

        if name == ("Windows.Foundation.Collections", "IIterable`1") {
            iterable = Some(interface);
        }
    }

    match iterable {
        None => TokenStream::new(),
        Some(interface) => {
            let constraints = def.gen_constraints(gen);
            let item = interface.def.generics[0].gen_name(gen);
            let name = def.gen_name(gen);

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
