use super::*;

impl win32::Win32Items<'_> {
    pub(super) fn lower_implementable_interfaces(
        &self,
        layout: Layout,
        projection: Projection,
    ) -> Result<
        (
            BTreeMap<Entity<TypeDef>, NativeInterface>,
            BTreeSet<(String, String)>,
        ),
        Error,
    > {
        let mut interfaces = BTreeMap::new();
        let mut implementable = BTreeSet::new();
        if projection.is_sys() {
            return Ok((interfaces, implementable));
        }

        for namespace in &self.selection.namespaces {
            for (entity, _) in &namespace.interfaces {
                interfaces.insert(
                    *entity,
                    NativeInterface::lower(
                        self.database,
                        &self.catalogs.dependencies,
                        self.database.definition(*entity).unwrap(),
                        &self.catalogs.interface_bases,
                    )?,
                );
            }
        }
        loop {
            let mut changed = false;
            for namespace in &self.selection.namespaces {
                for (entity, members) in &namespace.interfaces {
                    let definition = self.database.definition(*entity).unwrap();
                    let interface = &interfaces[entity];
                    let base_selected = interface.base_name().is_some_and(|(namespace, name)| {
                        implementable.contains(&(namespace.to_string(), name.to_string()))
                    });
                    if if layout.is_package() {
                        interface.can_implement_package(members, base_selected)
                    } else {
                        interface.can_implement(members, base_selected)
                    } {
                        changed |= implementable
                            .insert((namespace.name.clone(), definition.name()?.into()));
                    }
                }
            }
            if !changed {
                break;
            }
        }
        Ok((interfaces, implementable))
    }
}
