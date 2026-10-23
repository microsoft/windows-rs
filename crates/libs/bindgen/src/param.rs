use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Param {
    pub def: Option<MethodParam>,
    pub name: String,
    pub ty: Type,
    pub native_reference: bool,
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

    pub fn is_convertible_with_reader(&self, reader: &Reader) -> bool {
        self.is_input_only() && self.projection_type(reader).is_convertible()
    }

    pub fn projection_type(&self, reader: &Reader) -> Type {
        self.ty.projection_type(reader)
    }

    pub fn projection_pointee(&self, reader: &Reader) -> Type {
        self.ty.projection_pointee(reader)
    }

    /// Returns the inner type for input `IReference<T>` parameters exposed as `Option<T>`.
    pub fn ireference_inner(&self, reader: &Reader) -> Option<&Type> {
        if !self.is_input_only() {
            return None;
        }
        self.ty.ireference_inner_for_sugar(reader)
    }

    /// Returns whether metadata marks this parameter as input-only.
    pub fn is_input_only(&self) -> bool {
        self.def
            .is_some_and(|def| def.direction() == windows_metadata::reader::ParamDirection::Input)
    }

    pub fn has_direction(&self) -> bool {
        self.def.is_some_and(|def| {
            def.direction() != windows_metadata::reader::ParamDirection::Unspecified
        })
    }

    /// Returns whether metadata marks this parameter as `Output` without `Input`.
    pub fn is_metadata_output_only(&self) -> bool {
        self.def
            .is_some_and(|def| def.direction() == windows_metadata::reader::ParamDirection::Output)
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

    pub fn has_buffer_attributes(&self) -> bool {
        self.attributes().any(|attribute| {
            matches!(
                attribute.name(),
                "BufferExtentAttribute" | "NativeArrayInfoAttribute" | "MemorySizeAttribute"
            )
        })
    }

    pub fn has_unresolved_contracts(&self) -> bool {
        self.attributes().any(|attribute| {
            attribute.namespace() == "Windows.Win32.Metadata"
                && attribute.name() == "NativeSalAttribute"
        })
    }

    pub fn is_explicit_retval_candidate(&self, reader: &Reader) -> bool {
        self.is_retval_candidate(reader, true)
    }

    pub fn is_heuristic_retval_candidate(&self, reader: &Reader) -> bool {
        self.is_retval_candidate(reader, false)
    }

    fn is_retval_candidate(&self, reader: &Reader, explicit: bool) -> bool {
        if self.native_reference && !explicit {
            return false;
        }
        let ty = self.projection_type(reader);

        if !ty.is_pointer() {
            return false;
        }

        if !explicit && ty.is_void() {
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

        if self.has_buffer_attributes() || self.has_unresolved_contracts() {
            return false;
        }

        // Void-pointee and size limits are only heuristics for unmarked trailing pointers. An
        // explicit retval preserves its existing value projection.
        explicit || ty.deref().size(reader) <= 16
    }

    pub fn write_ident(&self) -> TokenStream {
        to_ident(&self.name.to_lowercase())
    }
}
