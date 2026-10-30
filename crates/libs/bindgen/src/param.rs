use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Param {
    pub def: Option<MethodParam>,
    pub name: String,
    pub ty: Type,
    /// Unspecified native direction whose type passes no caller-writable storage.
    pub implied_input: bool,
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

    /// Returns whether metadata marks this parameter as input-only, or its unspecified direction
    /// is implied input by its native type.
    pub fn is_input_only(&self) -> bool {
        self.implied_input
            || self.def.is_some_and(|def| {
                def.direction() == windows_metadata::reader::ParamDirection::Input
            })
    }

    pub fn has_direction(&self) -> bool {
        self.implied_input
            || self.def.is_some_and(|def| {
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
        self.buffer_attributes().next().is_some()
    }

    fn buffer_attributes(
        &self,
    ) -> impl Iterator<Item = windows_metadata::reader::Attribute<'static>> + '_ {
        self.attributes()
            .filter(|attribute| attribute.name() == "BufferExtentAttribute")
    }

    pub fn has_query_buffer_attributes(&self) -> bool {
        use windows_metadata::{BufferAccess, BufferCount, BufferExtent, BufferPhase};

        let returned_pointer = self.attributes().any(|attribute| {
            attribute.namespace() == "Windows.Win32.Metadata"
                && attribute.name() == "NativeReturnedPointerAttribute"
        });
        self.buffer_attributes().any(|attribute| {
            if attribute.namespace() != "Windows.Win32.Metadata"
                || attribute.name() != "BufferExtentAttribute"
                || !returned_pointer
            {
                return true;
            }
            let Some(extent) = BufferExtent::from_values(&attribute.value()) else {
                return true;
            };
            !(extent.access == BufferAccess::Read
                && extent.phase == BufferPhase::Success
                && extent.indirect
                && extent.count == BufferCount::Unknown)
        })
    }

    pub fn has_unresolved_contracts(&self) -> bool {
        self.attributes().any(|attribute| {
            attribute.namespace() == "Windows.Win32.Metadata"
                && attribute.name() == "NativeSalAttribute"
        })
    }

    fn returned_pointer_flag(&self, name: &str) -> bool {
        self.attributes().any(|attribute| {
            attribute.namespace() == "Windows.Win32.Metadata"
                && attribute.name() == "NativeReturnedPointerAttribute"
                && attribute
                    .value()
                    .iter()
                    .any(|(key, value)| key == name && *value == Value::Bool(true))
        })
    }

    /// Whether `NullableAttribute` permits a null returned pointer.
    pub fn result_may_be_null(&self) -> bool {
        self.attributes().any(|attribute| {
            attribute.namespace() == "Windows.Win32.Metadata"
                && attribute.name() == "NullableAttribute"
        })
    }

    pub fn result_null_on_failure(&self) -> bool {
        self.returned_pointer_flag("NullOnFailure")
    }

    pub fn write_result_type(&self, config: &Config) -> TokenStream {
        let ty = self.projection_pointee(config.reader);
        let name = ty.write_name(config);
        if self.result_may_be_null() && ty.is_interface_with_reader(config.reader) {
            quote! { Option<#name> }
        } else {
            name
        }
    }

    pub fn write_interface_result(&self) -> TokenStream {
        if self.result_may_be_null() {
            quote! {
                if result__.is_null() {
                    None
                } else {
                    Some(windows_core::Interface::from_raw(result__))
                }
            }
        } else {
            quote! { windows_core::imp::Type::from_abi(result__) }
        }
    }

    pub fn write_query_result_map(&self) -> TokenStream {
        let value = self.write_interface_result();
        if self.result_may_be_null() {
            quote! { map(|| #value) }
        } else {
            quote! { and_then(|| #value) }
        }
    }

    pub fn write_result_map(&self, reader: &Reader) -> TokenStream {
        let ty = self.projection_pointee(reader).projection_type(reader);
        if ty.is_interface_with_reader(reader) {
            self.write_query_result_map()
        } else {
            ty.write_result_map(reader)
        }
    }

    /// Only an explicit `[retval]` annotation projects the trailing parameter as the result.
    pub fn is_retval_candidate(&self, reader: &Reader) -> bool {
        if !self.is_retval_attribute() || !self.projection_type(reader).is_pointer() {
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

        !self.has_buffer_attributes() && !self.has_unresolved_contracts()
    }

    pub fn write_ident(&self) -> TokenStream {
        to_ident(&self.name.to_lowercase())
    }
}
