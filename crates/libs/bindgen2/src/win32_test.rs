use super::*;

impl win32::Win32Items<'_> {
    pub(crate) fn native_types(&self) -> impl Iterator<Item = Result<NativeType, Error>> + '_ {
        self.selection.namespaces.iter().flat_map(|namespace| {
            namespace.types.iter().map(|entity| {
                NativeType::lower_filtered(
                    self.database,
                    &self.catalogs.dependencies,
                    self.database.definition(*entity).unwrap(),
                    &self.catalogs.nested,
                    self.enum_variants(*entity),
                )
            })
        })
    }

    /// Lowers constants in deterministic namespace and name order.
    pub(crate) fn constants(&self) -> impl Iterator<Item = Result<Constant, Error>> + '_ {
        self.selection.namespaces.iter().flat_map(|namespace| {
            namespace.constants.iter().map(|entity| {
                let field = self.database.field(*entity).unwrap();
                Constant::lower(
                    self.database,
                    &self.catalogs.dependencies,
                    field,
                    &namespace.name,
                    field.name().unwrap(),
                )
            })
        })
    }

    /// Lowers functions in deterministic namespace and name order.
    pub(crate) fn functions(&self) -> impl Iterator<Item = Result<Function, Error>> + '_ {
        self.selection.namespaces.iter().flat_map(|namespace| {
            namespace.functions.iter().map(|entity| {
                let method = self.database.method(*entity).unwrap();
                Function::lower(
                    self.database,
                    &self.catalogs.dependencies,
                    method,
                    &namespace.name,
                    method.name().unwrap(),
                )
            })
        })
    }

    /// Lowers native delegates in deterministic namespace and name order.
    pub(crate) fn delegates(&self) -> impl Iterator<Item = Result<Delegate, Error>> + '_ {
        self.selection.namespaces.iter().flat_map(|namespace| {
            namespace.delegates.iter().map(|entity| {
                Delegate::lower(
                    self.database,
                    &self.catalogs.dependencies,
                    self.database.definition(*entity).unwrap(),
                )
            })
        })
    }

    /// Lowers native interfaces in deterministic namespace and name order.
    pub(crate) fn interfaces(&self) -> impl Iterator<Item = Result<NativeInterface, Error>> + '_ {
        self.selection.namespaces.iter().flat_map(|namespace| {
            namespace.interfaces.iter().map(|(entity, _)| {
                NativeInterface::lower(
                    self.database,
                    &self.catalogs.dependencies,
                    self.database.definition(*entity).unwrap(),
                    &self.catalogs.interface_bases,
                )
            })
        })
    }
}
