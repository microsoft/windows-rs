use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeMap;

struct Item {
    name: String,
    kind: u8,
    tokens: TokenStream,
}

#[derive(Default)]
struct Module {
    items: Vec<Item>,
    nested: BTreeMap<String, Self>,
}

impl Generator {
    /// Renders all currently supported items in the requested layout.
    pub fn render(&self, layout: Layout) -> Result<TokenStream, Error> {
        self.render_projection(layout, self.projection)
    }

    pub(super) fn render_projection(
        &self,
        layout: Layout,
        projection: Projection,
    ) -> Result<TokenStream, Error> {
        let mut modules = BTreeMap::<String, Vec<Item>>::new();
        let values = self.lower_values();
        for item in self.values() {
            let definition = item.definition();
            let namespace = definition.namespace()?;
            let name = definition.name()?;
            modules
                .entry(namespace.to_string())
                .or_default()
                .push(Item {
                    name: name.to_string(),
                    kind: 0,
                    tokens: values.write_context(namespace, name, layout, projection)?,
                });
        }
        for entry in self
            .winrt
            .iter()
            .filter(|entry| entry.kind == WinrtKind::Delegate)
        {
            let definition = self.shared.database.definition(entry.entity).unwrap();
            let namespace = definition.namespace()?;
            let metadata_name = definition.name()?;
            let name = metadata_name
                .split_once('`')
                .map_or(metadata_name, |(name, _)| name);
            let model = winrt_delegate::Delegate::lower(
                &self.shared.database,
                definition,
                &format!("{namespace}.{metadata_name}"),
            )?;
            modules
                .entry(namespace.to_string())
                .or_default()
                .push(Item {
                    name: name.to_string(),
                    kind: 0,
                    tokens: model.write(values, namespace, layout, projection)?,
                });
        }
        for entry in self
            .winrt
            .iter()
            .filter(|entry| entry.kind == WinrtKind::Class)
        {
            let definition = self.shared.database.definition(entry.entity).unwrap();
            let namespace = definition.namespace()?;
            let name = definition.name()?;
            let model = winrt_class::Class::lower(
                &self.shared.database,
                definition,
                &self.shared.interface_relationships,
                &format!("{namespace}.{name}"),
            )?;
            modules
                .entry(namespace.to_string())
                .or_default()
                .push(Item {
                    name: name.to_string(),
                    kind: 0,
                    tokens: model.write(
                        values,
                        namespace,
                        layout,
                        projection,
                        self.members(entry.entity),
                    )?,
                });
        }
        for entry in self
            .winrt
            .iter()
            .filter(|entry| entry.kind == WinrtKind::Interface)
        {
            let definition = self.shared.database.definition(entry.entity).unwrap();
            let namespace = definition.namespace()?;
            let metadata_name = definition.name()?;
            let name = trim_generic_arity(metadata_name);
            let model = winrt_interface::Interface::lower(
                &self.shared.database,
                definition,
                &self.shared.interface_relationships,
                &format!("{namespace}.{metadata_name}"),
            )?;
            modules
                .entry(namespace.to_string())
                .or_default()
                .push(Item {
                    name: name.to_string(),
                    kind: 0,
                    tokens: model.write(
                        values,
                        namespace,
                        layout,
                        projection,
                        self.members(entry.entity),
                        self.implements(entry.entity),
                    )?,
                });
        }

        self.win32_items()
            .render(layout, projection, |namespace, name, kind, tokens| {
                modules
                    .entry(namespace.to_string())
                    .or_default()
                    .push(Item {
                        name: name.to_string(),
                        kind,
                        tokens,
                    });
            })?;

        let mut root = Module::default();
        match layout {
            Layout::Modules => {
                for (namespace, items) in modules {
                    let mut module = &mut root;
                    if !namespace.is_empty() {
                        for name in namespace.split('.') {
                            module = module.nested.entry(name.to_string()).or_default();
                        }
                    }
                    module.items.extend(items);
                }
            }
            Layout::Flat => {
                let mut names = BTreeMap::<String, String>::new();
                for (namespace, items) in modules {
                    for item in items {
                        if let Some(first_namespace) = names.get(&item.name)
                            && first_namespace != &namespace
                        {
                            return Err(Error::FlatNameCollision {
                                name: item.name,
                                first_namespace: first_namespace.clone(),
                                second_namespace: namespace,
                            });
                        }
                        names.insert(item.name.clone(), namespace.clone());
                        root.items.push(item);
                    }
                }
            }
        }
        Ok(root.write())
    }
}

impl Module {
    fn write(mut self) -> TokenStream {
        self.items.sort_by(|left, right| {
            (left.kind != 3, &left.name, left.kind).cmp(&(right.kind != 3, &right.name, right.kind))
        });
        let items = self.items.into_iter().map(|item| item.tokens);
        let nested = self.nested.into_iter().map(|(name, module)| {
            let name = tokens::ident(&name);
            let module = module.write();
            quote! { pub mod #name { #module } }
        });
        quote! {
            #(#items)*
            #(#nested)*
        }
    }
}
