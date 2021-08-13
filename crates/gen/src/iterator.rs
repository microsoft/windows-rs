use super::*;

// Provides iterator support for the well-known WinRT collection interfaces and any classes or
// interfaces that implement any of these interfaces. It also favors high-speed iteration and
// only falls back to IIterator<T> if nothing faster is available. VectorIterator and
// VectorViewIterator are faster iterators than IIterator<T> because they only require a single
// vcall per iteration whereas IIterator<T> requires two.
pub fn gen_iterator(def: &TypeDef, interfaces: &[InterfaceInfo], gen: &Gen) -> TokenStream {
    match def.type_name() {
        // If the type is IIterator<T> then simply implement the Iterator trait over top.
        TypeName::IIterator => {
            return quote! {
                impl<T: ::windows::RuntimeType> ::std::iter::Iterator for IIterator<T> {
                    type Item = T;

                    fn next(&mut self) -> ::std::option::Option<Self::Item> {
                        let result = self.Current().ok();

                        if result.is_some() {
                            self.MoveNext().ok()?;
                        }

                        result
                    }
                }
            };
        }
        // If the type is IIterable<T> then implement the IntoIterator trait and rely on the resulting
        // IIterator<T> returned by first() to implement the Iterator trait.
        TypeName::IIterable => {
            return quote! {
                impl<T: ::windows::RuntimeType> ::std::iter::IntoIterator for IIterable<T> {
                    type Item = T;
                    type IntoIter = IIterator<Self::Item>;

                    fn into_iter(self) -> Self::IntoIter {
                        self.First().unwrap()
                    }
                }
                impl<'a, T: ::windows::RuntimeType> ::std::iter::IntoIterator for &'a IIterable<T> {
                    type Item = T;
                    type IntoIter = IIterator<Self::Item>;

                    fn into_iter(self) -> Self::IntoIter {
                        self.First().unwrap()
                    }
                }
            };
        }
        // If the type is IVectorView<T> then provide the VectorViewIterator fast iterator.
        TypeName::IVectorView => {
            return quote! {
                pub struct VectorViewIterator<T: ::windows::RuntimeType + 'static> {
                    vector: IVectorView<T>,
                    current: u32,
                    size: u32,
                }

                impl<T: ::windows::RuntimeType> VectorViewIterator<T> {
                    pub fn new(vector: IVectorView<T>) -> Self {
                        let size = vector.Size().unwrap();
                        Self { vector, current: 0, size }
                    }
                }

                impl<T: ::windows::RuntimeType> ::std::iter::Iterator for VectorViewIterator<T> {
                    type Item = T;

                    fn next(&mut self) -> ::std::option::Option<Self::Item> {
                        if self.current >= self.size {
                            return None;
                        }

                        let result = self.vector.GetAt(self.current);
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
        TypeName::IVector => {
            return quote! {
                pub struct VectorIterator<T: ::windows::RuntimeType + 'static> {
                    vector: IVector<T>,
                    current: u32,
                    size: u32,
                }

                impl<T: ::windows::RuntimeType> VectorIterator<T> {
                    pub fn new(vector: IVector<T>) -> Self {
                        let size = vector.Size().unwrap();
                        Self { vector, current: 0, size }
                    }
                }

                impl<T: ::windows::RuntimeType> ::std::iter::Iterator for VectorIterator<T> {
                    type Item = T;

                    fn next(&mut self) -> ::std::option::Option<Self::Item> {
                        if self.current >= self.size {
                            return None;
                        }

                        let result = self.vector.GetAt(self.current);
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
        _ => {}
    }

    let mut iterable = None;
    let wfc = gen.namespace("Windows.Foundation.Collections");

    // If the class or interface is not one of the well-known collection interfaces, we then see whether it
    // implements any one of them. Here is where we favor IVectorView/IVector over IIterable.
    for interface in interfaces {
        match interface.def.type_name() {
            TypeName::IVectorView => {
                let constraints = def.gen_constraints();
                let item = interface.def.generics[0].gen_name(gen);
                let name = gen_type_name(def, gen);

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
            TypeName::IVector => {
                let constraints = def.gen_constraints();
                let item = interface.def.generics[0].gen_name(gen);
                let name = gen_type_name(def, gen);

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
            TypeName::IIterable => {
                iterable = Some(interface);
            }
            _ => {}
        }
    }

    match iterable {
        None => TokenStream::new(),
        Some(interface) => {
            let constraints = def.gen_constraints();
            let item = interface.def.generics[0].gen_name(gen);
            let name = gen_type_name(def, gen);

            quote! {
               impl<#constraints> ::std::iter::IntoIterator for #name {
                    type Item = #item;
                    type IntoIter = #wfc IIterator<Self::Item>;

                    fn into_iter(self) -> Self::IntoIter {
                        self.First().unwrap()
                    }
                }
                impl<'a, #constraints> ::std::iter::IntoIterator for &'a #name {
                    type Item = #item;
                    type IntoIter = #wfc IIterator<Self::Item>;

                    fn into_iter(self) -> Self::IntoIter {
                        self.First().unwrap()
                    }
                }
            }
        }
    }
}
