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
            };
        }
        // If the type is IIterable<T> then implement the IntoIterator trait and rely on the resulting
        // IIterator<T> returned by first() to implement the Iterator trait.
        TypeName::IIterable => {
            return quote! {
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
            };
        }
        // If the type is IVectorView<T> then provide the VectorViewIterator fast iterator.
        TypeName::IVectorView => {
            return quote! {
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
            };
        }
        TypeName::IVector => {
            return quote! {
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
            };
        }
        _ => {}
    }

    let mut iterable = None;
    let wfc = gen.namespace("Windows.Foundation.Collections");

    let features = if gen.root.is_empty() {
        TokenStream::new()
    } else {
        quote! { #[cfg(all(feature = "Foundation_Collections"))] }
    };

    // If the class or interface is not one of the well-known collection interfaces, we then see whether it
    // implements any one of them. Here is where we favor IVectorView/IVector over IIterable.
    for interface in interfaces {
        match interface.def.type_name() {
            TypeName::IVectorView => {
                let constraints = gen_constraints(def);
                let item = gen_name(&interface.def.generics[0], gen);
                let name = gen_type_name(def, gen);

                return quote! {
                    #features
                    impl<#constraints> ::core::iter::IntoIterator for #name {
                        type Item = #item;
                        type IntoIter = #wfc VectorViewIterator<Self::Item>;

                        fn into_iter(self) -> Self::IntoIter {
                            ::core::iter::IntoIterator::into_iter(&self)
                        }
                    }
                    #features
                    impl<#constraints> ::core::iter::IntoIterator for &#name {
                        type Item = #item;
                        type IntoIter = #wfc VectorViewIterator<Self::Item>;

                        fn into_iter(self) -> Self::IntoIter {
                            #wfc VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
                        }
                    }
                };
            }
            TypeName::IVector => {
                let constraints = gen_constraints(def);
                let item = gen_name(&interface.def.generics[0], gen);
                let name = gen_type_name(def, gen);

                return quote! {
                    #features
                    impl<#constraints> ::core::iter::IntoIterator for #name {
                        type Item = #item;
                        type IntoIter = #wfc VectorIterator<Self::Item>;

                        fn into_iter(self) -> Self::IntoIter {
                            ::core::iter::IntoIterator::into_iter(&self)
                        }
                    }
                    #features
                    impl<#constraints> ::core::iter::IntoIterator for &#name {
                        type Item = #item;
                        type IntoIter = #wfc VectorIterator<Self::Item>;

                        fn into_iter(self) -> Self::IntoIter {
                            #wfc VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
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
            let constraints = gen_constraints(def);
            let item = gen_name(&interface.def.generics[0], gen);
            let name = gen_type_name(def, gen);

            quote! {
                #features
               impl<#constraints> ::core::iter::IntoIterator for #name {
                    type Item = #item;
                    type IntoIter = #wfc IIterator<Self::Item>;

                    fn into_iter(self) -> Self::IntoIter {
                        ::core::iter::IntoIterator::into_iter(&self)
                    }
                }
                #features
                impl<#constraints> ::core::iter::IntoIterator for &#name {
                    type Item = #item;
                    type IntoIter = #wfc IIterator<Self::Item>;

                    fn into_iter(self) -> Self::IntoIter {
                        // TODO: not sure how to avoid this unwrap, although it should always succeed in practice.
                        self.First().unwrap()
                    }
                }
            }
        }
    }
}
