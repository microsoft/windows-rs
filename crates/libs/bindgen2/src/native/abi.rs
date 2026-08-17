use super::*;
use std::collections::BTreeSet;

#[derive(Clone, Copy)]
enum AbiLayoutModel {
    Maximum,
    Legacy,
}

impl Type {
    pub(crate) fn producer_borrows_input(&self, database: &Database) -> Result<bool, Error> {
        if self.is_bstr()
            || self.is_hstring()
            || self.is_pcstr()
            || self.is_pcwstr()
            || self.is_guid()
        {
            return Ok(true);
        }
        let Self::Named {
            namespace, name, ..
        } = self
        else {
            return Ok(false);
        };
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            if definition.category()? != TypeCategory::Struct {
                continue;
            }
            if !definition.has_attribute("NativeTypedefAttribute")? {
                return Ok(true);
            }
            let fields = definition.fields()?.collect::<Vec<_>>();
            let [field] = fields.as_slice() else {
                return Ok(true);
            };
            let field = Self::lower(
                database,
                field.entity().file(),
                &format!("{namespace}.{name}"),
                field.signature()?,
            )?;
            if field.is_const_string()
                || field.mutable_string_pointer()
                || field.is_bstr()
                || field.is_hstring()
            {
                return Ok(true);
            }
            return Ok(!field.producer_primitive(database, &mut BTreeSet::new())?);
        }
        Ok(false)
    }

    pub(crate) fn producer_primitive(
        &self,
        database: &Database,
        stack: &mut BTreeSet<(String, String)>,
    ) -> Result<bool, Error> {
        let Self::Named {
            namespace, name, ..
        } = self
        else {
            return self.is_primitive(database);
        };
        if self
            .canonical()
            .is_some_and(canonical::Type::is_native_primitive)
        {
            return Ok(true);
        }
        let key = (namespace.clone(), name.clone());
        if !stack.insert(key.clone()) {
            return Ok(false);
        }
        let mut result = false;
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            match definition.category()? {
                TypeCategory::Enum | TypeCategory::Delegate => {
                    result = true;
                    break;
                }
                TypeCategory::Struct if definition.has_attribute("NativeTypedefAttribute")? => {
                    let fields = definition.fields()?.collect::<Vec<_>>();
                    let [field] = fields.as_slice() else {
                        continue;
                    };
                    if field.name()? != "Value" {
                        continue;
                    }
                    let field =
                        Self::lower(database, field.entity().file(), name, field.signature()?)?;
                    if field.producer_primitive(database, stack)? {
                        result = true;
                        break;
                    }
                }
                _ => {}
            }
        }
        stack.remove(&key);
        Ok(result)
    }

    pub(crate) fn resolves_to_delegate(
        &self,
        database: &Database,
        stack: &mut BTreeSet<(String, String)>,
    ) -> Result<bool, Error> {
        let Self::Named {
            namespace, name, ..
        } = self
        else {
            return Ok(false);
        };
        let key = (namespace.clone(), name.clone());
        if !stack.insert(key.clone()) {
            return Ok(false);
        }
        let mut result = false;
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            match definition.category()? {
                TypeCategory::Delegate => {
                    result = true;
                    break;
                }
                TypeCategory::Struct if definition.has_attribute("NativeTypedefAttribute")? => {
                    let fields = definition.fields()?.collect::<Vec<_>>();
                    let [field] = fields.as_slice() else {
                        continue;
                    };
                    let field =
                        Self::lower(database, field.entity().file(), name, field.signature()?)?;
                    if field.resolves_to_delegate(database, stack)? {
                        result = true;
                        break;
                    }
                }
                _ => {}
            }
        }
        stack.remove(&key);
        Ok(result)
    }

    pub(crate) fn is_delegate(&self, database: &Database) -> Result<bool, Error> {
        let Self::Named {
            namespace, name, ..
        } = self
        else {
            return Ok(false);
        };
        for entity in database.type_definitions(namespace, name) {
            if database.definition(*entity).unwrap().category()? == TypeCategory::Delegate {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub(crate) fn needs_output_pointer_cast(
        &self,
        database: &Database,
        stack: &mut BTreeSet<(String, String)>,
    ) -> Result<bool, Error> {
        if let Self::Pointer { mutable, element } = self {
            return Ok(*mutable && **element == Self::Void);
        }
        let Self::Named {
            namespace, name, ..
        } = self
        else {
            return Ok(false);
        };
        if is_core_projection(namespace, name) {
            return Ok(false);
        }
        let key = (namespace.clone(), name.clone());
        if !stack.insert(key.clone()) {
            return Ok(false);
        }
        let mut result = false;
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            if definition.category()? != TypeCategory::Struct
                || !definition.has_attribute("NativeTypedefAttribute")?
            {
                continue;
            }
            let fields = definition.fields()?.collect::<Vec<_>>();
            let [field] = fields.as_slice() else {
                continue;
            };
            if field.name()? != "Value" {
                continue;
            }
            let field = Self::lower(database, field.entity().file(), name, field.signature()?)?;
            if matches!(field, Self::Pointer { .. })
                || field.needs_output_pointer_cast(database, stack)?
            {
                result = true;
                break;
            }
        }
        stack.remove(&key);
        Ok(result)
    }

    pub(crate) fn is_mutable_void_double_pointer(&self) -> bool {
        matches!(
            self,
            Self::Pointer {
                mutable: true,
                element,
            } if matches!(
                element.as_ref(),
                Self::Pointer {
                    mutable: true,
                    element,
                } if element.as_ref() == &Self::Void
            )
        )
    }

    pub(crate) fn is_wrapper_underlying(&self, database: &Database) -> Result<bool, Error> {
        Ok(self.is_mutable_void_double_pointer()
            || (self.is_primitive(database)?
                && !self.resolves_to_delegate(database, &mut BTreeSet::new())?
                && !matches!(
                    self,
                    Self::Pointer { element, .. } if element.as_ref() != &Self::Void
                )))
    }

    pub(crate) fn is_noncanonical_pointer_alias(&self, database: &Database) -> Result<bool, Error> {
        Ok(self.pointer_alias(database)?.is_some())
    }

    pub(crate) fn pointer_alias(&self, database: &Database) -> Result<Option<Self>, Error> {
        let Self::Named {
            namespace, name, ..
        } = self
        else {
            return Ok(None);
        };
        if is_core_projection(namespace, name) {
            return Ok(None);
        }
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            if definition.category()? != TypeCategory::Struct
                || !definition.has_attribute("NativeTypedefAttribute")?
            {
                continue;
            }
            let fields = definition
                .fields()?
                .filter_map(|field| (!field.is_literal().ok()?).then_some(field))
                .collect::<Vec<_>>();
            let [field] = fields.as_slice() else {
                continue;
            };
            let ty = Self::lower(
                database,
                field.entity().file(),
                definition.name()?,
                field.signature()?,
            )?;
            if matches!(
                ty,
                Self::Pointer { ref element, .. } if element.as_ref() != &Self::Void
            ) && !ty.is_mutable_void_double_pointer()
            {
                return Ok(Some(ty));
            }
        }
        Ok(None)
    }

    pub(crate) fn resolved_pointer_alias(
        &self,
        database: &Database,
    ) -> Result<Option<Self>, Error> {
        self.resolved_pointer_alias_inner(database, &mut BTreeSet::new())
    }

    fn resolved_pointer_alias_inner(
        &self,
        database: &Database,
        stack: &mut BTreeSet<(String, String)>,
    ) -> Result<Option<Self>, Error> {
        if let Some(ty) = self.pointer_alias(database)? {
            return Ok(Some(ty));
        }
        let Self::Named {
            namespace, name, ..
        } = self
        else {
            return Ok(None);
        };
        let key = (namespace.clone(), name.clone());
        if !stack.insert(key.clone()) {
            return Ok(None);
        }
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            if definition.category()? != TypeCategory::Struct
                || !definition.has_attribute("NativeTypedefAttribute")?
            {
                continue;
            }
            let fields = definition.fields()?.collect::<Vec<_>>();
            let [field] = fields.as_slice() else {
                continue;
            };
            let ty = Self::lower(
                database,
                field.entity().file(),
                definition.name()?,
                field.signature()?,
            )?;
            if let Some(ty) = ty.resolved_pointer_alias_inner(database, stack)? {
                stack.remove(&key);
                return Ok(Some(ty));
            }
        }
        stack.remove(&key);
        Ok(None)
    }

    pub(crate) fn is_primitive(&self, database: &Database) -> Result<bool, Error> {
        self.is_primitive_inner(database, &mut BTreeSet::new())
    }

    pub(crate) fn is_integer(&self, database: &Database) -> Result<bool, Error> {
        self.is_integer_inner(database, &mut BTreeSet::new())
    }

    pub(crate) fn is_newtype(&self, database: &Database) -> Result<bool, Error> {
        let Self::Named {
            namespace, name, ..
        } = self
        else {
            return Ok(false);
        };
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            match definition.category()? {
                TypeCategory::Enum => return Ok(true),
                TypeCategory::Struct => {
                    let mut fields = Vec::new();
                    for field in definition.fields()? {
                        if !field.is_literal()? {
                            fields.push(field);
                        }
                    }
                    let [field] = fields.as_slice() else {
                        continue;
                    };
                    if field.name()? != "Value" {
                        continue;
                    }
                    let ty =
                        Self::lower(database, field.entity().file(), name, field.signature()?)?;
                    if ty.is_primitive(database)?
                        && !matches!(
                            ty,
                            Self::Pointer { ref element, .. }
                                if element.as_ref() != &Self::Void
                        )
                    {
                        return Ok(true);
                    }
                }
                _ => {}
            }
        }
        Ok(false)
    }

    fn is_integer_inner(
        &self,
        database: &Database,
        stack: &mut BTreeSet<(String, String)>,
    ) -> Result<bool, Error> {
        let Self::Named {
            namespace, name, ..
        } = self
        else {
            return Ok(matches!(
                self,
                Self::Char
                    | Self::I8
                    | Self::U8
                    | Self::I16
                    | Self::U16
                    | Self::I32
                    | Self::U32
                    | Self::I64
                    | Self::U64
                    | Self::ISize
                    | Self::USize
            ));
        };
        let key = (namespace.clone(), name.clone());
        if !stack.insert(key.clone()) {
            return Ok(false);
        }
        let mut result = false;
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            match definition.category()? {
                TypeCategory::Enum => {
                    result = true;
                    break;
                }
                TypeCategory::Struct if definition.has_attribute("NativeTypedefAttribute")? => {
                    let fields = definition.fields()?.collect::<Vec<_>>();
                    let [field] = fields.as_slice() else {
                        continue;
                    };
                    let field =
                        Self::lower(database, field.entity().file(), name, field.signature()?)?;
                    if field.is_integer_inner(database, stack)? {
                        result = true;
                        break;
                    }
                }
                _ => {}
            }
        }
        stack.remove(&key);
        Ok(result)
    }

    fn is_primitive_inner(
        &self,
        database: &Database,
        stack: &mut BTreeSet<(String, String)>,
    ) -> Result<bool, Error> {
        let Self::Named {
            namespace, name, ..
        } = self
        else {
            return Ok(matches!(
                self,
                Self::Boolean
                    | Self::Char
                    | Self::I8
                    | Self::U8
                    | Self::I16
                    | Self::U16
                    | Self::I32
                    | Self::U32
                    | Self::I64
                    | Self::U64
                    | Self::F32
                    | Self::F64
                    | Self::ISize
                    | Self::USize
                    | Self::Pointer { .. }
            ));
        };
        if self
            .canonical()
            .is_some_and(canonical::Type::is_native_primitive)
        {
            return Ok(true);
        }
        let key = (namespace.clone(), name.clone());
        if !stack.insert(key.clone()) {
            return Ok(false);
        }
        let mut result = false;
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            match definition.category()? {
                TypeCategory::Enum | TypeCategory::Delegate => {
                    result = true;
                    break;
                }
                TypeCategory::Struct => {}
                _ => {}
            }
        }
        stack.remove(&key);
        Ok(result)
    }

    pub(crate) fn is_hresult(&self) -> bool {
        self.canonical().is_some_and(canonical::Type::is_hresult)
    }

    pub(crate) fn is_void_alias(&self, database: &Database) -> Result<bool, Error> {
        self.is_void_alias_inner(database, &mut BTreeSet::new())
    }

    fn is_void_alias_inner(
        &self,
        database: &Database,
        stack: &mut BTreeSet<(String, String)>,
    ) -> Result<bool, Error> {
        if self == &Self::Void {
            return Ok(true);
        }
        let Self::Named {
            namespace, name, ..
        } = self
        else {
            return Ok(false);
        };
        let key = (namespace.clone(), name.clone());
        if !stack.insert(key.clone()) {
            return Ok(false);
        }
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            if definition.category()? != TypeCategory::Struct
                || !definition.has_attribute("NativeTypedefAttribute")?
            {
                continue;
            }
            let fields = definition.fields()?.collect::<Vec<_>>();
            let [field] = fields.as_slice() else {
                continue;
            };
            let ty = Self::lower(
                database,
                field.entity().file(),
                definition.name()?,
                field.signature()?,
            )?;
            if ty.is_void_alias_inner(database, stack)? {
                stack.remove(&key);
                return Ok(true);
            }
        }
        stack.remove(&key);
        Ok(false)
    }

    pub(crate) fn is_guid(&self) -> bool {
        self.canonical().is_some_and(canonical::Type::is_guid)
    }

    pub(crate) fn is_hresult_package(&self) -> bool {
        self.is_hresult()
    }

    pub(crate) fn is_ntstatus(&self) -> bool {
        self.canonical().is_some_and(canonical::Type::is_ntstatus)
    }

    pub(crate) fn is_bool(&self) -> bool {
        self.canonical().is_some_and(canonical::Type::is_bool)
    }

    pub(crate) fn is_bstr(&self) -> bool {
        self.canonical().is_some_and(canonical::Type::is_bstr)
    }

    pub(crate) fn is_hstring(&self) -> bool {
        self.canonical().is_some_and(canonical::Type::is_hstring)
    }

    pub(crate) fn is_pcwstr(&self) -> bool {
        self.canonical().is_some_and(canonical::Type::is_pcwstr)
    }

    pub(crate) fn is_pstr(&self) -> bool {
        self.canonical().is_some_and(canonical::Type::is_pstr)
    }

    pub(crate) fn is_pcstr(&self) -> bool {
        self.canonical().is_some_and(canonical::Type::is_pcstr)
    }

    pub(crate) fn is_const_string(&self) -> bool {
        self.canonical()
            .is_some_and(canonical::Type::is_const_string)
    }

    pub(crate) fn is_indirect_return(&self, database: &Database) -> Result<bool, Error> {
        if self.uses_winrt_projection() {
            return Ok(false);
        }
        if self.is_hresult() {
            return Ok(false);
        }
        let Self::Named {
            namespace, name, ..
        } = self
        else {
            return Ok(false);
        };
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            if definition.category()? == TypeCategory::Struct {
                if !definition.has_attribute("NativeTypedefAttribute")? {
                    return Ok(true);
                }
                let fields = definition.fields()?.collect::<Vec<_>>();
                if !matches!(fields.as_slice(), [field] if field.name()? == "Value") {
                    return Ok(true);
                }
                let field = Self::lower(
                    database,
                    fields[0].entity().file(),
                    &format!("{namespace}.{name}"),
                    fields[0].signature()?,
                )?;
                return field.is_indirect_return(database);
            }
        }
        Ok(false)
    }

    #[cfg(test)]
    pub(crate) fn exceeds_retval_limit(&self, database: &Database) -> Result<bool, Error> {
        Ok(self
            .abi_layout_model(database, &mut BTreeSet::new(), AbiLayoutModel::Maximum)?
            .0
            > 16)
    }

    pub(crate) fn exceeds_retval_limit_legacy(&self, database: &Database) -> Result<bool, Error> {
        Ok(self
            .abi_layout_model(database, &mut BTreeSet::new(), AbiLayoutModel::Legacy)?
            .0
            > 16)
    }

    pub(super) fn abi_layout(
        &self,
        database: &Database,
        stack: &mut BTreeSet<(String, String)>,
    ) -> Result<(usize, usize), Error> {
        self.abi_layout_model(database, stack, AbiLayoutModel::Maximum)
    }

    fn abi_layout_model(
        &self,
        database: &Database,
        stack: &mut BTreeSet<(String, String)>,
        model: AbiLayoutModel,
    ) -> Result<(usize, usize), Error> {
        Ok(match self {
            Self::Void => (0, 1),
            Self::I8 | Self::U8 => (1, 1),
            Self::I16 | Self::U16 => (2, 2),
            Self::I64 | Self::U64 | Self::F64 => (8, 8),
            Self::Pointer { .. }
            | Self::Interface { .. }
            | Self::String
            | Self::ISize
            | Self::USize
                if matches!(model, AbiLayoutModel::Maximum) =>
            {
                (8, 8)
            }
            Self::Array { element, len } => {
                let (size, align) = element.abi_layout_model(database, stack, model)?;
                let align = if matches!(model, AbiLayoutModel::Legacy) {
                    align.saturating_mul(*len)
                } else {
                    align
                };
                (size.saturating_mul(*len), align)
            }
            Self::Named {
                namespace, name, ..
            } => {
                let key = (namespace.clone(), name.clone());
                if !stack.insert(key.clone()) {
                    return Ok((0, 1));
                }
                let mut result = None::<(usize, usize)>;
                for entity in database.type_definitions(namespace, name) {
                    let definition = database.definition(*entity).unwrap();
                    if definition.category()? != TypeCategory::Struct {
                        continue;
                    }
                    let explicit = definition
                        .type_attributes()?
                        .contains(TypeAttributes::EXPLICIT_LAYOUT);
                    let packing = definition
                        .layout()?
                        .map(|layout| layout.packing_size())
                        .transpose()?
                        .filter(|packing| *packing != 0)
                        .map(usize::from);
                    let mut definition_layout = (0usize, 1usize);
                    for field in definition.fields()? {
                        if field.is_literal()? {
                            continue;
                        }
                        let (field_size, mut field_align) =
                            Self::lower(database, field.entity().file(), name, field.signature()?)?
                                .abi_layout_model(database, stack, model)?;
                        if let Some(packing) = packing {
                            field_align = field_align.min(packing);
                        }
                        if explicit {
                            definition_layout.0 = definition_layout.0.max(field_size);
                        } else {
                            definition_layout.0 = align_up(definition_layout.0, field_align);
                            definition_layout.0 = definition_layout.0.saturating_add(field_size);
                        }
                        definition_layout.1 = definition_layout.1.max(field_align);
                    }
                    result = Some(result.map_or(definition_layout, |result| {
                        (
                            result.0.max(definition_layout.0),
                            result.1.max(definition_layout.1),
                        )
                    }));
                }
                stack.remove(&key);
                result.unwrap_or((4, 4))
            }
            _ => (4, 4),
        })
    }
}
