use super::*;

pub fn writer(writer: &Writer, def: TypeDef, generics: &[Type], ident: &TokenStream, constraints: &TokenStream, _phantoms: &TokenStream, cfg: &Cfg) -> TokenStream {
    match writer.reader.type_def_type_name(def) {
        // If the type is IIterator<T> then simply implement the Iterator trait over top.
        TypeName::IIterator => {
            return quote! {
                impl<T: ::windows_core::RuntimeType> ::core::iter::Iterator for IIterator<T> {
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
                impl<T: ::windows_core::RuntimeType> ::core::iter::IntoIterator for IIterable<T> {
                    type Item = T;
                    type IntoIter = IIterator<Self::Item>;

                    fn into_iter(self) -> Self::IntoIter {
                        ::core::iter::IntoIterator::into_iter(&self)
                    }
                }
                impl<T: ::windows_core::RuntimeType> ::core::iter::IntoIterator for &IIterable<T> {
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
                pub struct VectorViewIterator<T: ::windows_core::RuntimeType + 'static> {
                    vector: ::core::option::Option<IVectorView<T>>,
                    current: u32,
                }

                impl<T: ::windows_core::RuntimeType> VectorViewIterator<T> {
                    pub fn new(vector: ::core::option::Option<IVectorView<T>>) -> Self {
                        Self { vector, current: 0 }
                    }
                }

                impl<T: ::windows_core::RuntimeType> ::core::iter::Iterator for VectorViewIterator<T> {
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

                impl<T: ::windows_core::RuntimeType> ::core::iter::IntoIterator for IVectorView<T> {
                    type Item = T;
                    type IntoIter = VectorViewIterator<Self::Item>;

                    fn into_iter(self) -> Self::IntoIter {
                        ::core::iter::IntoIterator::into_iter(&self)
                    }
                }
                impl<T: ::windows_core::RuntimeType> ::core::iter::IntoIterator for &IVectorView<T> {
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
                pub struct VectorIterator<T: ::windows_core::RuntimeType + 'static> {
                    vector: ::core::option::Option<IVector<T>>,
                    current: u32,
                }

                impl<T: ::windows_core::RuntimeType> VectorIterator<T> {
                    pub fn new(vector: ::core::option::Option<IVector<T>>) -> Self {
                        Self { vector, current: 0 }
                    }
                }

                impl<T: ::windows_core::RuntimeType> ::core::iter::Iterator for VectorIterator<T> {
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

                impl<T: ::windows_core::RuntimeType> ::core::iter::IntoIterator for IVector<T> {
                    type Item = T;
                    type IntoIter = VectorIterator<Self::Item>;

                    fn into_iter(self) -> Self::IntoIter {
                        ::core::iter::IntoIterator::into_iter(&self)
                    }
                }
                impl<T: ::windows_core::RuntimeType> ::core::iter::IntoIterator for &IVector<T> {
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

    let wfc = writer.namespace("Windows.Foundation.Collections");
    let mut iterable = None;
    let interfaces = type_interfaces(writer.reader, &Type::TypeDef(def, generics.to_vec()));

    // If the class or interface is not one of the well-known collection interfaces, we then see whether it
    // implements any one of them. Here is where we favor IVectorView/IVector over IIterable.
    for interface in interfaces {
        if let Type::TypeDef(interface, interface_generics) = &interface.ty {
            match writer.reader.type_def_type_name(*interface) {
                TypeName::IVectorView => {
                    let item = writer.type_name(&interface_generics[0]);
                    let mut cfg = cfg.clone();
                    type_def_cfg_combine(writer.reader, *interface, interface_generics, &mut cfg);
                    let features = writer.cfg_features(&cfg);

                    return quote! {
                        #features
                        impl<#constraints> ::core::iter::IntoIterator for #ident {
                            type Item = #item;
                            type IntoIter = #wfc VectorViewIterator<Self::Item>;

                            fn into_iter(self) -> Self::IntoIter {
                                ::core::iter::IntoIterator::into_iter(&self)
                            }
                        }
                        #features
                        impl<#constraints> ::core::iter::IntoIterator for &#ident {
                            type Item = #item;
                            type IntoIter = #wfc VectorViewIterator<Self::Item>;

                            fn into_iter(self) -> Self::IntoIter {
                                #wfc VectorViewIterator::new(::windows_core::ComInterface::cast(self).ok())
                            }
                        }
                    };
                }
                TypeName::IVector => {
                    let item = writer.type_name(&interface_generics[0]);
                    let mut cfg = cfg.clone();
                    type_def_cfg_combine(writer.reader, *interface, interface_generics, &mut cfg);
                    let features = writer.cfg_features(&cfg);

                    return quote! {
                        #features
                        impl<#constraints> ::core::iter::IntoIterator for #ident {
                            type Item = #item;
                            type IntoIter = #wfc VectorIterator<Self::Item>;

                            fn into_iter(self) -> Self::IntoIter {
                                ::core::iter::IntoIterator::into_iter(&self)
                            }
                        }
                        #features
                        impl<#constraints> ::core::iter::IntoIterator for &#ident {
                            type Item = #item;
                            type IntoIter = #wfc VectorIterator<Self::Item>;

                            fn into_iter(self) -> Self::IntoIter {
                                #wfc VectorIterator::new(::windows_core::ComInterface::cast(self).ok())
                            }
                        }
                    };
                }
                TypeName::IIterable => {
                    iterable = Some((*interface, interface_generics.to_vec()));
                }
                _ => {}
            }
        }
    }

    match iterable {
        None => TokenStream::new(),
        Some((interface, interface_generics)) => {
            let item = writer.type_name(&interface_generics[0]);
            let mut cfg = cfg.clone();
            type_def_cfg_combine(writer.reader, interface, &interface_generics, &mut cfg);
            let features = writer.cfg_features(&cfg);

            quote! {
                #features
                impl<#constraints> ::core::iter::IntoIterator for #ident {
                    type Item = #item;
                    type IntoIter = #wfc IIterator<Self::Item>;

                    fn into_iter(self) -> Self::IntoIter {
                        ::core::iter::IntoIterator::into_iter(&self)
                    }
                }
                #features
                impl<#constraints> ::core::iter::IntoIterator for &#ident {
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
