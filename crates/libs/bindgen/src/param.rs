use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Param {
    pub def: Option<MethodParam>,
    pub name: String,
    pub ty: Type,
}

impl std::ops::Deref for Param {
    type Target = Type;

    fn deref(&self) -> &Self::Target {
        &self.ty
    }
}

impl Param {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn attributes(
        &self,
    ) -> impl Iterator<Item = windows_metadata::reader::Attribute<'static>> + '_ {
        self.def.into_iter().flat_map(|def| def.attributes())
    }

    pub fn has_attribute(&self, name: &str) -> bool {
        self.def.is_some_and(|def| def.has_attribute(name))
    }

    pub fn is_convertible(&self) -> bool {
        self.is_input_only() && self.ty.is_convertible()
    }

    /// Returns the inner type for input `IReference<T>` parameters exposed as `Option<T>`.
    pub fn ireference_inner(&self, reader: &Reader) -> Option<&Type> {
        if !self.is_input_only() {
            return None;
        }
        self.ty.ireference_inner_for_sugar(reader)
    }

    /// Returns whether the Rust projection treats this parameter as input-only.
    ///
    /// An unspecified direction uses the existing input fallback. `InputOutput` takes the
    /// output-capable branch so mutable pointers and slices stay mutable.
    pub fn is_input_only(&self) -> bool {
        matches!(
            self.def.map_or(
                windows_metadata::reader::ParamDirection::Unspecified,
                |def| def.direction()
            ),
            windows_metadata::reader::ParamDirection::Unspecified
                | windows_metadata::reader::ParamDirection::Input
        )
    }

    /// Returns whether Rust's projection permits omitted storage for an optional or reserved
    /// parameter.
    pub fn is_optional_or_reserved(&self) -> bool {
        self.def
            .is_some_and(|def| def.is_optional() || def.is_reserved())
    }

    pub fn is_retval_attribute(&self) -> bool {
        self.def.is_some_and(|def| def.is_retval_attribute())
    }

    pub fn buffer_relationship(&self) -> Option<BufferRelationship> {
        self.def?.buffer_relationship()
    }

    pub fn is_explicit_retval_candidate(&self, reader: &Reader) -> bool {
        self.is_retval_candidate(reader, true)
    }

    pub fn is_heuristic_retval_candidate(&self, reader: &Reader) -> bool {
        self.is_retval_candidate(reader, false)
    }

    fn is_retval_candidate(&self, reader: &Reader, explicit: bool) -> bool {
        if !self.ty.is_pointer() {
            return false;
        }

        if !explicit && self.ty.is_void() {
            return false;
        }

        let Some(def) = self.def else {
            return false;
        };

        if def.direction() != windows_metadata::reader::ParamDirection::Output
            || def.is_optional()
            || def.is_reserved()
        {
            return false;
        }

        for attribute in self.attributes() {
            if matches!(
                attribute.name(),
                "NativeArrayInfoAttribute" | "MemorySizeAttribute"
            ) {
                return false;
            }
        }

        // Void-pointee and size limits are only heuristics for unmarked trailing pointers. An
        // explicit retval preserves its existing value projection.
        if !explicit && self.ty.deref().size(reader) > 16 {
            return false;
        }

        true
    }

    pub fn write_ident(&self) -> TokenStream {
        to_ident(&self.name.to_lowercase())
    }
}
