use super::*;

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct TypeMap(HashMap<TypeName, HashSet<Type>>, HashMap<Type, i32>);

pub trait Dependencies {
    fn combine(&self, dependencies: &mut TypeMap, reader: &Reader);

    fn dependencies(&self, reader: &Reader) -> TypeMap {
        let mut dependencies = TypeMap::new();
        self.combine(&mut dependencies, reader);
        dependencies
    }
}

pub fn associated_enums<'a>(row: impl HasAttributes<'a>, reader: &Reader) -> Vec<Type> {
    let mut result = Vec::new();
    for attribute in row.attributes().filter(|attribute| {
        attribute.name() == "AssociatedEnumAttribute"
            && attribute.ctor().parent().namespace() == "Windows.Win32.Metadata"
    }) {
        let values = attribute.value();
        let [(_, Value::TypeName(name))] = values.as_slice() else {
            panic!("AssociatedEnumAttribute requires one System.Type argument");
        };
        let types: Vec<_> = reader.with_full_name(&name.namespace, &name.name).collect();
        assert!(
            !types.is_empty() && types.iter().all(|ty| matches!(ty, Type::CppEnum(_))),
            "AssociatedEnumAttribute target `{}.{}` must be a Win32 enum",
            name.namespace,
            name.name
        );
        for ty in &types {
            let Type::CppEnum(enumeration) = ty else {
                unreachable!()
            };
            if !enumeration.def.has_attribute("ScopedEnumAttribute") {
                for field in enumeration
                    .def
                    .fields()
                    .filter(|field| field.flags().contains(FieldAttributes::Literal))
                {
                    result.extend(
                        reader
                            .with_full_name(&name.namespace, field.name())
                            .filter(|ty| matches!(ty, Type::CppConst(_))),
                    );
                }
            }
        }
        result.extend(types);
    }
    result
}

impl std::ops::Deref for TypeMap {
    type Target = HashMap<TypeName, HashSet<Type>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TypeMap {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds `ty` and returns whether its dependencies still need to be walked.
    pub fn insert(&mut self, ty: Type) -> bool {
        let name = ty.type_name();
        self.0.entry(name).or_default().insert(ty)
    }

    /// Adds `ty` as reached from items on `arches` and returns whether any architecture is new.
    pub fn insert_reached(&mut self, ty: Type, arches: i32) -> bool {
        let reached = self.1.entry(ty.clone()).or_default();
        if *reached & arches == arches {
            return false;
        }
        *reached |= arches;
        self.insert(ty);
        true
    }

    /// Returns the architectures whose selected items reach `ty`; unrecorded types are unrestricted.
    pub fn reached_arches(&self, ty: &Type) -> i32 {
        self.1.get(ty).copied().unwrap_or(ALL_ARCHES)
    }

    pub fn combine(&mut self, other: &Self) {
        for (name, types) in &other.0 {
            let set = self.0.entry(*name).or_default();
            for ty in types {
                if let Some(arches) = self.1.get_mut(ty) {
                    *arches |= other.reached_arches(ty);
                } else if let Some(arches) = other.1.get(ty)
                    && !set.contains(ty)
                {
                    self.1.insert(ty.clone(), *arches);
                }
                set.insert(ty.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combined_dependencies_preserve_reached_architectures() {
        let mut precise = TypeMap::new();
        precise.insert_reached(Type::GUID, 1);
        let mut combined = TypeMap::new();
        combined.combine(&precise);
        assert_eq!(combined.reached_arches(&Type::GUID), 1);

        let mut other = TypeMap::new();
        other.insert_reached(Type::GUID, 2);
        combined.combine(&other);
        assert_eq!(combined.reached_arches(&Type::GUID), 3);

        let mut broad = TypeMap::new();
        broad.insert(Type::GUID);
        combined.combine(&broad);
        assert_eq!(combined.reached_arches(&Type::GUID), ALL_ARCHES);
        broad.combine(&precise);
        assert_eq!(broad.reached_arches(&Type::GUID), ALL_ARCHES);
    }
}
