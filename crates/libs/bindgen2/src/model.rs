use super::*;
use proc_macro2::TokenStream;
use std::collections::{BTreeMap, BTreeSet};

/// An owned projected WinRT value type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    /// A WinRT enum.
    Enum(Enum),
    /// A WinRT struct.
    Struct(Struct),
}

/// The global value graph required by recursive struct semantics.
pub struct Values {
    namespaces: BTreeMap<String, BTreeMap<String, Value>>,
    len: usize,
}

impl Generator {
    /// Lowers all selected enum and struct definitions into owned models.
    pub fn lower_values(&self) -> Result<Values, Error> {
        Values::lower(self)
    }
}

impl Values {
    fn lower(generator: &Generator) -> Result<Self, Error> {
        let mut namespaces = BTreeMap::<String, BTreeMap<String, Value>>::new();
        let mut len = 0;
        for item in generator.values() {
            let definition = item.definition();
            let namespace = definition.namespace()?.to_string();
            let name = definition.name()?.to_string();
            let full_name = format!("{namespace}.{name}");
            let value = match item.kind() {
                ValueKind::Enum => {
                    Value::Enum(Enum::lower(generator.database(), definition, &full_name)?)
                }
                ValueKind::Struct => {
                    Value::Struct(Struct::lower(generator.database(), definition, &full_name)?)
                }
            };
            if namespaces
                .entry(namespace)
                .or_default()
                .insert(name, value)
                .is_some()
            {
                return Err(Error::DuplicateValue(full_name));
            }
            len += 1;
        }
        Ok(Self { namespaces, len })
    }

    /// Returns the number of lowered value types.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns whether no value types were lowered.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns a lowered value by metadata namespace and name.
    pub fn get(&self, namespace: &str, name: &str) -> Option<&Value> {
        self.namespaces
            .get(namespace)
            .and_then(|types| types.get(name))
    }

    /// Iterates values in namespace/name order.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str, &Value)> {
        self.namespaces.iter().flat_map(|(namespace, types)| {
            types
                .iter()
                .map(move |(name, value)| (namespace.as_str(), name.as_str(), value))
        })
    }

    /// Renders a lowered value with rich WinRT projection policy.
    pub fn write(&self, namespace: &str, name: &str) -> Result<TokenStream, Error> {
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
            .ok_or_else(|| Error::InvalidValue {
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
            None => Err(Error::InvalidValue {
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
            None => Err(Error::InvalidValue {
                name: format!("{namespace}.{name}"),
                message: "referenced value was not selected",
            }),
        };
        stack.remove(&key);
        result
    }
}
