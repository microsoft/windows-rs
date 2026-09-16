use super::*;

/// Type references and item names indexed from WinMD files.
pub struct MetadataReferences {
    types: BTreeMap<String, TypeReference>,
    reference_types: BTreeSet<String>,
    excluded_types: BTreeSet<String>,
    excluded_functions: BTreeSet<String>,
    excluded_constants: BTreeSet<String>,
}

impl MetadataReferences {
    /// Indexes WinMD files for use during RDL emission.
    pub fn new(files: impl IntoIterator<Item = windows_metadata::reader::File>) -> Self {
        let index = windows_metadata::reader::Index::new(files.into_iter().collect());
        let mut types = BTreeMap::new();
        let mut ambiguous = BTreeSet::new();
        for (namespace, name, ty) in index.iter() {
            let kind = match ty.category() {
                windows_metadata::reader::TypeCategory::Enum => TypeReferenceKind::Enum,
                windows_metadata::reader::TypeCategory::Interface => TypeReferenceKind::Interface,
                _ => TypeReferenceKind::Type,
            };
            let mut reference = TypeReference::new(namespace, name, kind);
            if kind == TypeReferenceKind::Enum {
                reference = reference.with_enum_members(ty.fields().map(|field| field.name()));
            }
            if types
                .insert(name.to_string(), reference.clone())
                .is_some_and(|existing| existing != reference)
            {
                ambiguous.insert(name.to_string());
            }
        }
        types.retain(|name, _| !ambiguous.contains(name));
        let reference_types = types.keys().cloned().collect();

        let mut excluded_types = BTreeSet::new();
        let mut excluded_functions = BTreeSet::new();
        let mut excluded_constants = BTreeSet::new();
        for (_, name, item) in index.iter_items() {
            match item {
                windows_metadata::reader::Item::Type(_) => {
                    excluded_types.insert(name.to_string());
                }
                windows_metadata::reader::Item::Fn(_) => {
                    excluded_functions.insert(name.to_string());
                }
                windows_metadata::reader::Item::Const(_) => {
                    excluded_constants.insert(name.to_string());
                }
            }
        }

        Self {
            types,
            reference_types,
            excluded_types,
            excluded_functions,
            excluded_constants,
        }
    }

    /// Returns the unambiguous type references.
    pub fn types(&self) -> &BTreeMap<String, TypeReference> {
        &self.types
    }

    /// Returns all type names present in the indexed metadata.
    pub fn excluded_types(&self) -> &BTreeSet<String> {
        &self.excluded_types
    }

    /// Returns all function names present in the indexed metadata.
    pub fn excluded_functions(&self) -> &BTreeSet<String> {
        &self.excluded_functions
    }

    /// Returns all constant names present in the indexed metadata.
    pub fn excluded_constants(&self) -> &BTreeSet<String> {
        &self.excluded_constants
    }

    /// Applies every indexed item name as an emission exclusion.
    pub fn apply_exclusions<'a>(&'a self, options: &mut EmitOptions<'a>) {
        options.excluded_types = Some(&self.excluded_types);
        options.excluded_functions = Some(&self.excluded_functions);
        options.excluded_constants = Some(&self.excluded_constants);
    }

    /// Applies exclusions that have unambiguous external type references.
    pub fn apply_reference_exclusions<'a>(&'a self, options: &mut EmitOptions<'a>) {
        options.excluded_types = Some(&self.reference_types);
        options.excluded_functions = Some(&self.excluded_functions);
        options.excluded_constants = Some(&self.excluded_constants);
    }
}
