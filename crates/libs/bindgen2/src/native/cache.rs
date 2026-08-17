use super::*;

impl DependencyCache {
    pub(crate) fn new(
        database: &Database,
        bases: &BTreeMap<Entity<TypeDef>, Vec<(String, String)>>,
        sys_namespaces: BTreeSet<String>,
    ) -> Result<Self, Error> {
        let mut interface_bases = BTreeMap::<(String, String), BTreeSet<(String, String)>>::new();
        for (entity, bases) in bases {
            let definition = database.definition(*entity).unwrap();
            interface_bases
                .entry((
                    definition.namespace()?.to_string(),
                    definition.name()?.to_string(),
                ))
                .or_default()
                .extend(bases.iter().cloned());
        }
        Ok(Self {
            values: RwLock::default(),
            interfaces: RwLock::default(),
            interface_bases,
            sys_namespaces,
        })
    }

    pub(crate) fn package_sys_override(
        &self,
        dependencies: &BTreeSet<(String, String)>,
    ) -> Option<BTreeSet<(String, String)>> {
        if dependencies
            .iter()
            .all(|(namespace, _)| self.supports_package_sys_namespace(namespace))
        {
            return None;
        }
        Some(
            dependencies
                .iter()
                .filter(|(namespace, _)| self.supports_package_sys_namespace(namespace))
                .cloned()
                .collect(),
        )
    }

    fn supports_package_sys_namespace(&self, namespace: &str) -> bool {
        namespace == "Windows.Win32"
            || !namespace.starts_with("Windows.Win32.")
            || self.sys_namespaces.contains(namespace)
    }

    pub(crate) fn interface_dependencies(
        &self,
        database: &Database,
        namespace: &str,
        name: &str,
    ) -> Result<InterfaceDependencies, Error> {
        let key = (namespace.to_string(), name.to_string());
        if let Some(dependencies) = self.interfaces.read().unwrap().get(&key) {
            return Ok(dependencies.clone());
        }
        let mut dependencies = InterfaceDependencies::default();
        let owner = format!("{namespace}.{name}");
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            for method in definition.methods()? {
                let signature = native_signature::Signature::lower(database, self, method, &owner)?;
                dependencies
                    .package
                    .extend(signature.package_dependencies().iter().cloned());
                dependencies
                    .manifest
                    .extend(signature.manifest_dependencies());
            }
        }
        self.interfaces
            .write()
            .unwrap()
            .insert(key, dependencies.clone());
        Ok(dependencies)
    }

    fn direct(
        &self,
        database: &Database,
        namespace: &str,
        name: &str,
    ) -> Result<BTreeSet<(String, String)>, Error> {
        let key = (namespace.to_string(), name.to_string());
        if let Some(dependencies) = self.values.read().unwrap().get(&key) {
            return Ok(dependencies.clone());
        }
        let mut dependencies = BTreeSet::new();
        for entity in database.type_definitions(namespace, name) {
            Type::collect_definition_direct_dependencies(
                database,
                database.definition(*entity).unwrap(),
                namespace,
                name,
                &mut dependencies,
            )?;
        }
        if let Some(bases) = self.interface_bases.get(&key) {
            dependencies.extend(bases.iter().cloned());
        }
        self.values
            .write()
            .unwrap()
            .insert(key, dependencies.clone());
        Ok(dependencies)
    }

    pub(super) fn expand(
        &self,
        database: &Database,
        namespace: &str,
        name: &str,
        stack: &mut BTreeSet<(String, String)>,
        dependencies: &mut BTreeSet<(String, String)>,
    ) -> Result<(), Error> {
        let key = (namespace.to_string(), name.to_string());
        if is_core_projection(namespace, name) || !stack.insert(key.clone()) {
            return Ok(());
        }
        for (namespace, name) in self.direct(database, namespace, name)? {
            dependencies.insert((namespace.clone(), name.clone()));
            self.expand(database, &namespace, &name, stack, dependencies)?;
        }
        stack.remove(&key);
        Ok(())
    }

    pub(super) fn expand_interface_bases(
        &self,
        namespace: &str,
        name: &str,
        stack: &mut BTreeSet<(String, String)>,
        dependencies: &mut BTreeSet<(String, String)>,
    ) {
        let key = (namespace.to_string(), name.to_string());
        if !stack.insert(key.clone()) {
            return;
        }
        if let Some(bases) = self.interface_bases.get(&key) {
            for (namespace, name) in bases {
                dependencies.insert((namespace.clone(), name.clone()));
                self.expand_interface_bases(namespace, name, stack, dependencies);
            }
        }
        stack.remove(&key);
    }
}
