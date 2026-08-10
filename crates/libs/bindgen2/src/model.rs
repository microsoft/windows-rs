use super::*;
use proc_macro2::TokenStream;
use std::collections::{BTreeMap, BTreeSet};

/// An owned projected WinRT value type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Value {
    /// A WinRT enum.
    Enum(Enum),
    /// A WinRT struct.
    Struct(Struct),
}

/// The global value graph required by recursive struct semantics.
pub(crate) struct Values {
    namespaces: BTreeMap<String, BTreeMap<String, Value>>,
}

impl Generator {
    /// Returns the shared lowered WinRT value catalog.
    pub(crate) fn lower_values(&self) -> &Values {
        &self.shared.values
    }
}

impl Values {
    pub(crate) fn lower(
        database: &Database,
        entries: &[(String, String, WinrtEntry)],
    ) -> Result<Self, Error> {
        let mut namespaces = BTreeMap::<String, BTreeMap<String, Value>>::new();
        for (namespace, name, entry) in entries {
            let definition = database.definition(entry.entity).unwrap();
            let full_name = format!("{namespace}.{name}");
            let value = match entry.kind {
                WinrtKind::Enum => Value::Enum(Enum::lower(database, definition, &full_name)?),
                WinrtKind::Struct => {
                    Value::Struct(Struct::lower(database, definition, &full_name)?)
                }
                WinrtKind::Delegate | WinrtKind::Interface => continue,
            };
            if namespaces
                .entry(namespace.clone())
                .or_default()
                .insert(name.clone(), value)
                .is_some()
            {
                return Err(Error::DuplicateValue(full_name));
            }
        }
        Ok(Self { namespaces })
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.namespaces.values().map(BTreeMap::len).sum()
    }

    /// Returns a lowered value by metadata namespace and name.
    pub(crate) fn get(&self, namespace: &str, name: &str) -> Option<&Value> {
        self.namespaces
            .get(namespace)
            .and_then(|types| types.get(name))
    }

    #[cfg(test)]
    pub(crate) fn iter(&self) -> impl Iterator<Item = (&str, &str, &Value)> {
        self.namespaces.iter().flat_map(|(namespace, types)| {
            types
                .iter()
                .map(move |(name, value)| (namespace.as_str(), name.as_str(), value))
        })
    }

    #[cfg(test)]
    pub(crate) fn write(&self, namespace: &str, name: &str) -> Result<TokenStream, Error> {
        self.write_context(namespace, name, Layout::Modules)
    }

    pub(super) fn write_context(
        &self,
        namespace: &str,
        name: &str,
        layout: Layout,
    ) -> Result<TokenStream, Error> {
        let value = self
            .get(namespace, name)
            .ok_or_else(|| Error::InvalidType {
                name: format!("{namespace}.{name}"),
                message: "value was not selected",
            })?;
        match value {
            Value::Enum(model) => model.write(self, namespace, name, layout),
            Value::Struct(model) => model.write(self, namespace, name, layout),
        }
    }

    pub(super) fn properties(
        &self,
        namespace: &str,
        name: &str,
        stack: &mut BTreeSet<(String, String)>,
    ) -> Result<ty::Properties, Error> {
        let key = (namespace.to_string(), name.to_string());
        if !stack.insert(key.clone()) {
            return Err(Error::RecursiveValue(format!("{namespace}.{name}")));
        }
        let result = match self.get(namespace, name) {
            Some(Value::Enum(_)) => Ok(ty::Properties {
                copyable: true,
                eq: true,
            }),
            Some(Value::Struct(model)) => {
                model.properties(self, stack, &format!("{namespace}.{name}"))
            }
            None => Err(Error::InvalidType {
                name: format!("{namespace}.{name}"),
                message: "referenced value was not selected",
            }),
        };
        stack.remove(&key);
        result
    }

    pub(super) fn signature(
        &self,
        namespace: &str,
        name: &str,
        stack: &mut BTreeSet<(String, String)>,
    ) -> Result<String, Error> {
        let key = (namespace.to_string(), name.to_string());
        if !stack.insert(key.clone()) {
            return Err(Error::RecursiveValue(format!("{namespace}.{name}")));
        }
        let result = match self.get(namespace, name) {
            Some(Value::Enum(model)) => model.signature(self, namespace, name, stack),
            Some(Value::Struct(model)) => model.runtime_signature(self, namespace, name, stack),
            None => Err(Error::InvalidType {
                name: format!("{namespace}.{name}"),
                message: "referenced value was not selected",
            }),
        };
        stack.remove(&key);
        result
    }
}
