use super::*;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub(super) struct ArtifactGraph {
    direct: BTreeMap<(String, String), BTreeSet<(String, String)>>,
}

impl ArtifactGraph {
    pub(super) fn new(
        entries: &[(String, String, WinrtEntry)],
        values: &Values,
        catalogs: &winrt_catalog::Catalogs,
    ) -> Result<Self, Error> {
        let mut direct = BTreeMap::<_, BTreeSet<_>>::new();
        for (namespace, name, entry) in entries {
            let dependencies = match entry.kind {
                WinrtKind::Enum => BTreeSet::new(),
                WinrtKind::Struct => match values.get(namespace, name) {
                    Some(Value::Struct(model)) => model.dependencies(),
                    _ => BTreeSet::new(),
                },
                WinrtKind::Delegate => catalogs
                    .delegate(entry.entity)
                    .direct_selection_dependencies(),
                WinrtKind::Interface => catalogs
                    .interface(entry.entity)
                    .direct_artifact_dependencies(),
                WinrtKind::Class => catalogs.class(entry.entity).direct_artifact_dependencies(),
            };
            direct
                .entry((namespace.clone(), name.clone()))
                .or_default()
                .extend(dependencies);
        }
        Ok(Self { direct })
    }

    pub(super) fn expand(
        &self,
        dependencies: &BTreeSet<(String, String)>,
    ) -> BTreeSet<(String, String)> {
        let mut result = dependencies.clone();
        let mut stack = BTreeSet::new();
        for dependency in dependencies {
            self.expand_one(dependency, &mut stack, &mut result);
        }
        result
    }

    fn expand_one(
        &self,
        dependency: &(String, String),
        stack: &mut BTreeSet<(String, String)>,
        result: &mut BTreeSet<(String, String)>,
    ) {
        if !stack.insert(dependency.clone()) {
            return;
        }
        if let Some(dependencies) = self.direct.get(dependency) {
            for dependency in dependencies {
                if result.insert(dependency.clone()) {
                    self.expand_one(dependency, stack, result);
                }
            }
        }
        stack.remove(dependency);
    }
}
