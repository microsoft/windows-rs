use super::*;
use std::collections::BTreeMap;

pub(super) struct Catalogs {
    delegates: BTreeMap<Entity<TypeDef>, winrt_delegate::Delegate>,
    interfaces: BTreeMap<Entity<TypeDef>, winrt_interface::Interface>,
    classes: BTreeMap<Entity<TypeDef>, winrt_class::Class>,
}

impl Catalogs {
    pub(super) fn new(
        database: &Database,
        entries: &[(String, String, WinrtEntry)],
        relationships: &BTreeMap<Entity<TypeDef>, Vec<InterfaceRelationship>>,
    ) -> Result<Self, Error> {
        let mut delegates = BTreeMap::new();
        let mut interfaces = BTreeMap::new();
        let mut classes = BTreeMap::new();
        for (namespace, name, entry) in entries {
            let definition = database.definition(entry.entity).unwrap();
            let owner = format!("{namespace}.{name}");
            match entry.kind {
                WinrtKind::Delegate => {
                    delegates.insert(
                        entry.entity,
                        winrt_delegate::Delegate::lower(database, definition, &owner)?,
                    );
                }
                WinrtKind::Interface => {
                    interfaces.insert(
                        entry.entity,
                        winrt_interface::Interface::lower(
                            database,
                            definition,
                            relationships,
                            &owner,
                        )?,
                    );
                }
                WinrtKind::Class => {
                    classes.insert(
                        entry.entity,
                        winrt_class::Class::lower(database, definition, relationships, &owner)?,
                    );
                }
                WinrtKind::Enum | WinrtKind::Struct => {}
            }
        }
        Ok(Self {
            delegates,
            interfaces,
            classes,
        })
    }

    pub(super) fn delegate(&self, entity: Entity<TypeDef>) -> &winrt_delegate::Delegate {
        self.delegates.get(&entity).unwrap()
    }

    pub(super) fn interface(&self, entity: Entity<TypeDef>) -> &winrt_interface::Interface {
        self.interfaces.get(&entity).unwrap()
    }

    pub(super) fn class(&self, entity: Entity<TypeDef>) -> &winrt_class::Class {
        self.classes.get(&entity).unwrap()
    }
}
