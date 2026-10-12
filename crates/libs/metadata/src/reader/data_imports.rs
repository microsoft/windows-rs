use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativeDataImportTarget {
    Name(String),
    Ordinal(u16),
}

/// COFF import-object kind, independent of native type qualifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeDataImportKind {
    Data,
    Constant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeDataImport {
    pub dll: String,
    pub target: NativeDataImportTarget,
    pub kind: NativeDataImportKind,
}

#[derive(Debug)]
pub struct NativeDataImportError(String);

impl std::fmt::Display for NativeDataImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for NativeDataImportError {}

fn data_attribute(attr: Attribute<'_>) -> bool {
    attr.namespace() == "Windows.Win32.Metadata" && attr.name() == "NativeDataImportAttribute"
}

impl Field<'_> {
    /// Identifies declaration-only imported data without inferring it from field flags.
    pub fn is_imported_data(&self) -> bool {
        self.attributes().any(data_attribute)
    }

    /// Decodes one explicit DLL/export-or-ordinal and COFF import-kind contract.
    pub fn data_import(&self) -> Result<Option<NativeDataImport>, NativeDataImportError> {
        let mut attributes = self.attributes().filter(|attr| data_attribute(*attr));
        let Some(attr) = attributes.next() else {
            return Ok(None);
        };
        if attributes.next().is_some() {
            return invalid("duplicate native data import descriptor");
        }
        let args = attr.value();
        if args.iter().any(|(name, _)| !name.is_empty()) {
            return invalid("native data import arguments must be positional");
        }
        let values: Vec<_> = args.into_iter().map(|(_, value)| value).collect();
        let [
            Value::Utf8(dll),
            Value::Utf8(name),
            Value::I32(ordinal),
            Value::I32(kind),
        ] = values.as_slice()
        else {
            return invalid("invalid native data import arguments");
        };
        if dll.is_empty() || dll.contains('\0') || name.contains('\0') {
            return invalid("native data import requires nonempty DLL and NUL-free names");
        }
        let target = if !name.is_empty() && *ordinal == -1 {
            NativeDataImportTarget::Name(name.clone())
        } else if name.is_empty() {
            let Ok(ordinal) = u16::try_from(*ordinal) else {
                return invalid("native data import ordinal must fit u16");
            };
            NativeDataImportTarget::Ordinal(ordinal)
        } else {
            return invalid("native data import requires exactly one export name or ordinal");
        };
        let kind = match kind {
            1 => NativeDataImportKind::Data,
            2 => NativeDataImportKind::Constant,
            _ => return invalid("native data import kind must be data or constant"),
        };
        Ok(Some(NativeDataImport {
            dll: dll.clone(),
            target,
            kind,
        }))
    }
}

impl Index {
    /// Checks declaration-only native data, without resolving DLLs or allocating native storage.
    pub fn validate_native_data_imports(
        &self,
        reference: Option<&Self>,
    ) -> Result<(), NativeDataImportError> {
        let mut fields = BTreeSet::new();
        for attr in self.all_attributes().filter(|attr| data_attribute(*attr)) {
            let HasAttribute::Field(field) = attr.parent() else {
                return invalid("native data import descriptor requires a field");
            };
            fields.insert(field);
        }
        for field in fields {
            field.data_import()?;
            let owner: TypeDef<'_> = field.parent_row(4);
            if owner.name() != "Apis"
                || owner.namespace().is_empty()
                || owner.category() != TypeCategory::Class
                || owner.flags().contains(TypeAttributes::WindowsRuntime)
                || owner.generic_params().next().is_some()
                || owner.arches() != 0
            {
                return invalid("native data imports require a neutral native Apis container");
            }
            if field.flags() != FieldAttributes::Public | FieldAttributes::Static
                || field.name().is_empty()
                || field.constant().is_some()
                || field.attributes().any(|attr| {
                    attr.namespace() == "Windows.Win32.Metadata" && attr.name() == "GuidAttribute"
                })
            {
                return invalid(
                    "native data imports require Public|Static without a value or storage flags",
                );
            }
            let arches = if field.arches() == 0 {
                owner.arches()
            } else {
                field.arches()
            };
            self.check_data_type(&field.ty(), false, arches, reference, &mut Vec::new())?;
        }
        Ok(())
    }

    fn check_data_type(
        &self,
        ty: &Type,
        pointee: bool,
        arches: i32,
        reference: Option<&Self>,
        visiting: &mut Vec<TypeName>,
    ) -> Result<(), NativeDataImportError> {
        match ty.unqualified() {
            Type::Void if pointee => Ok(()),
            Type::Bool
            | Type::Char
            | Type::I8
            | Type::U8
            | Type::I16
            | Type::U16
            | Type::I32
            | Type::U32
            | Type::I64
            | Type::U64
            | Type::F32
            | Type::F64
            | Type::ISize
            | Type::USize => Ok(()),
            Type::PtrMut(inner, _) | Type::PtrConst(inner, _) => {
                self.check_data_type(inner, true, arches, reference, visiting)
            }
            Type::RefMut(inner) | Type::RefConst(inner) | Type::ArrayFixed(inner, _) => {
                self.check_data_type(inner, false, arches, reference, visiting)
            }
            Type::ValueName(name) | Type::ClassName(name) if name.generics.is_empty() => {
                if visiting.contains(name) {
                    return invalid("cyclic native data typedef");
                }
                visiting.push(name.clone());
                for definition in self.get(&name.namespace, &name.name).chain(
                    reference
                        .into_iter()
                        .flat_map(|index| index.get(&name.namespace, &name.name)),
                ) {
                    let target_arches = definition.arches();
                    if arches != 0 && target_arches != 0 && arches & target_arches == 0 {
                        continue;
                    }
                    if definition.flags().contains(TypeAttributes::WindowsRuntime) {
                        return invalid("native data import type cannot be WinRT");
                    }
                    if matches!(ty.unqualified(), Type::ClassName(_))
                        && !matches!(
                            definition.category(),
                            TypeCategory::Interface | TypeCategory::Delegate
                        )
                    {
                        return invalid(
                            "native data reference type must be an interface or callback",
                        );
                    }
                    if definition.attributes().any(|attr| {
                        attr.namespace() == "Windows.Win32.Metadata"
                            && attr.name() == "NativeTypedefAttribute"
                    }) {
                        if definition.fields().len() != 1 {
                            return invalid("native data typedef requires one target field");
                        }
                        let underlying = definition.fields().next().unwrap().ty();
                        let target_arches = if arches == 0 { target_arches } else { arches };
                        self.check_data_type(
                            &underlying,
                            pointee,
                            target_arches,
                            reference,
                            visiting,
                        )?;
                    }
                }
                visiting.pop();
                Ok(())
            }
            _ => invalid("native data import requires a native nonvoid type"),
        }
    }
}

fn invalid<T>(message: &str) -> Result<T, NativeDataImportError> {
    Err(NativeDataImportError(message.to_string()))
}
