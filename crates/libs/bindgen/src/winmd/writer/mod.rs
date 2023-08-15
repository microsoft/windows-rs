mod blobs;
mod codes;
mod file;
mod strings;
mod tables;
mod traits;
mod r#type;

use super::*;
use blobs::Blobs;
pub use codes::*;
use metadata::imp::*;
pub use r#type::*;
use std::collections::HashMap;
use strings::Strings;
pub use tables::*;
use traits::*;

pub struct Writer {
    pub blobs: Blobs,
    pub strings: Strings,
    pub tables: Tables,
    pub scopes: HashMap<String, u32>,
    // TODO: is this faster than jsut using a single HashMap with a (String,String) key?
    pub type_refs: HashMap<String, HashMap<String, u32>>,
    pub type_specs: HashMap<Type, u32>,
}

impl Writer {
    pub fn new(name: &str) -> Self {
        let mut writer = Self {
            blobs: Default::default(),
            strings: Default::default(),
            tables: Default::default(),
            scopes: Default::default(),
            type_refs: Default::default(),
            type_specs: Default::default(),
        };

        writer.tables.TypeDef.push(TypeDef { TypeName: writer.strings.insert("<Module>"), ..Default::default() });

        let name = name.rsplit_once(&['/', '\\']).map_or(name, |(_, name)| name);

        writer.tables.Module.push(Module { Name: writer.strings.insert(name), Mvid: 1, ..Default::default() });

        let name = name.rsplit_once('.').map_or(name, |(_, name)| name);

        writer.tables.Assembly.push(Assembly {
            Name: writer.strings.insert(name),
            HashAlgId: 0x00008004,
            MajorVersion: 0xFF,
            MinorVersion: 0xFF,
            BuildNumber: 0xFF,
            RevisionNumber: 0xFF,
            Flags: metadata::AssemblyFlags::WindowsRuntime.0,
            ..Default::default()
        });

        // Some winmd parsers will fail to read without an `mscorlib` reference. The `insert_module_types` function will typically include it
        // automatically but a minimal `Module` tree may not add this dependency.
        writer.insert_scope("System");

        writer
    }

    pub fn into_stream(self) -> Vec<u8> {
        file::write(self.tables.into_stream(), self.strings.into_stream(), self.blobs.into_stream())
    }

    pub fn insert_method_sig(&mut self, call_flags: metadata::MethodCallAttributes, return_type: &Type, param_types: &[Type]) -> u32 {
        let mut blob = vec![call_flags.0];
        usize_blob(param_types.len(), &mut blob);
        self.type_blob(return_type, &mut blob);

        for ty in param_types {
            self.type_blob(ty, &mut blob);
        }

        self.blobs.insert(&blob)
    }

    pub fn insert_field_sig(&mut self, ty: &Type) -> u32 {
        // TODO: can either cache in Writer, like we do for scopes and type_refs, or regenerate each time.
        // Profile once we can stress test this with field/method signatures.

        let mut blob = vec![0x6]; // FIELD
        self.type_blob(ty, &mut blob);

        self.blobs.insert(&blob)
    }

    fn insert_scope(&mut self, namespace: &str) -> u32 {
        if let Some(scope) = self.scopes.get(namespace) {
            *scope
        } else if namespace == "System" {
            let scope = ResolutionScope::AssemblyRef(self.tables.AssemblyRef.push2(AssemblyRef {
                Name: self.strings.insert("mscorlib"),
                MajorVersion: 4,
                PublicKeyOrToken: self.blobs.insert(&[0xB7, 0x7A, 0x5C, 0x56, 0x19, 0x34, 0xE0, 0x89]), // TODO: comment on this
                ..Default::default()
            }))
            .encode();
            self.scopes.insert(namespace.to_string(), scope);
            scope
        } else {
            // TODO: may need to capture the original assembly info for external type_refs.
            let scope = ResolutionScope::AssemblyRef(self.tables.AssemblyRef.push2(AssemblyRef {
                Name: self.strings.insert(namespace),
                MajorVersion: 0xFF,
                MinorVersion: 0xFF,
                BuildNumber: 0xFF,
                RevisionNumber: 0xFF,
                Flags: metadata::AssemblyFlags::WindowsRuntime.0,
                ..Default::default()
            }))
            .encode();
            self.scopes.insert(namespace.to_string(), scope);
            scope
        }
    }

    pub fn insert_type_ref(&mut self, namespace: &str, name: &str) -> u32 {
        if let Some(key) = self.type_refs.get(namespace) {
            if let Some(reference) = key.get(name) {
                return *reference;
            }
        }

        let scope = self.insert_scope(namespace);

        let reference = TypeDefOrRef::TypeRef(self.tables.TypeRef.push2(TypeRef { TypeName: self.strings.insert(name), TypeNamespace: self.strings.insert(namespace), ResolutionScope: scope })).encode();
        self.type_refs.entry(namespace.to_string()).or_default().insert(name.to_string(), reference);
        reference
    }

    pub fn insert_type_spec(&mut self, ty: Type) -> u32 {
        if let Some(key) = self.type_specs.get(&ty) {
            return *key;
        }

        let mut blob = vec![];
        self.type_blob(&ty, &mut blob);
        let signature = self.blobs.insert(&blob);

        let reference = TypeDefOrRef::TypeSpec(self.tables.TypeSpec.push2(TypeSpec { Signature: signature })).encode();

        self.type_specs.insert(ty, reference);
        reference
    }

    fn type_blob(&mut self, ty: &Type, blob: &mut Vec<u8>) {
        match ty {
            Type::Void => blob.push(ELEMENT_TYPE_VOID),
            Type::Bool => blob.push(ELEMENT_TYPE_BOOLEAN),
            Type::Char => blob.push(ELEMENT_TYPE_CHAR),
            Type::I8 => blob.push(ELEMENT_TYPE_I1),
            Type::U8 => blob.push(ELEMENT_TYPE_U1),
            Type::I16 => blob.push(ELEMENT_TYPE_I2),
            Type::U16 => blob.push(ELEMENT_TYPE_U2),
            Type::I32 => blob.push(ELEMENT_TYPE_I4),
            Type::U32 => blob.push(ELEMENT_TYPE_U4),
            Type::I64 => blob.push(ELEMENT_TYPE_I8),
            Type::U64 => blob.push(ELEMENT_TYPE_U8),
            Type::F32 => blob.push(ELEMENT_TYPE_R4),
            Type::F64 => blob.push(ELEMENT_TYPE_R8),
            Type::ISize => blob.push(ELEMENT_TYPE_I),
            Type::USize => blob.push(ELEMENT_TYPE_U),
            Type::String => blob.push(ELEMENT_TYPE_STRING),
            Type::IInspectable => blob.push(ELEMENT_TYPE_OBJECT),
            Type::GUID => {
                let code = self.insert_type_ref("System", "Guid");
                blob.push(ELEMENT_TYPE_VALUETYPE);
                usize_blob(code as usize, blob);
            }
            Type::HRESULT => {
                let code = self.insert_type_ref("Windows.Foundation", "HResult");
                blob.push(ELEMENT_TYPE_VALUETYPE);
                usize_blob(code as usize, blob);
            }
            Type::TypeRef(ty) => {
                if !ty.generics.is_empty() {
                    blob.push(ELEMENT_TYPE_GENERICINST);
                }
                let code = self.insert_type_ref(&ty.namespace, &ty.name);
                blob.push(ELEMENT_TYPE_VALUETYPE);
                usize_blob(code as usize, blob);

                if !ty.generics.is_empty() {
                    usize_blob(ty.generics.len(), blob);

                    for ty in &ty.generics {
                        self.type_blob(ty, blob);
                    }
                }
            }
            Type::BSTR => {
                let code = self.insert_type_ref("Windows.Win32.Foundation", "BSTR");
                blob.push(ELEMENT_TYPE_VALUETYPE);
                usize_blob(code as usize, blob);
            }
            Type::IUnknown => {
                let code = self.insert_type_ref("Windows.Win32.Foundation", "IUnknown");
                blob.push(ELEMENT_TYPE_VALUETYPE);
                usize_blob(code as usize, blob);
            }
            Type::PCWSTR | Type::PWSTR => {
                let code = self.insert_type_ref("Windows.Win32.Foundation", "PWSTR");
                blob.push(ELEMENT_TYPE_VALUETYPE);
                usize_blob(code as usize, blob);
            }
            Type::PCSTR | Type::PSTR => {
                let code = self.insert_type_ref("Windows.Win32.Foundation", "PSTR");
                blob.push(ELEMENT_TYPE_VALUETYPE);
                usize_blob(code as usize, blob);
            }
            Type::ConstRef(ty) => {
                usize_blob(ELEMENT_TYPE_CMOD_OPT as usize, blob);
                usize_blob(self.insert_type_ref("System.Runtime.CompilerServices", "IsConst") as usize, blob);
                usize_blob(ELEMENT_TYPE_BYREF as usize, blob);
                self.type_blob(ty, blob);
            }
            Type::WinrtArrayRef(ty) => {
                usize_blob(ELEMENT_TYPE_BYREF as usize, blob);
                usize_blob(ELEMENT_TYPE_SZARRAY as usize, blob);
                self.type_blob(ty, blob);
            }
            Type::WinrtArray(ty) => {
                usize_blob(ELEMENT_TYPE_SZARRAY as usize, blob);
                self.type_blob(ty, blob);
            }
            Type::Win32Array(ty, bounds) => {
                usize_blob(ELEMENT_TYPE_ARRAY as usize, blob);
                self.type_blob(ty, blob);
                usize_blob(1, blob); // rank
                usize_blob(1, blob); // count
                usize_blob(*bounds, blob);
            }
            Type::TypeName => {
                let code = self.insert_type_ref("System", "Type");
                blob.push(ELEMENT_TYPE_CLASS);
                usize_blob(code as usize, blob);
            }
            Type::MutPtr(ty, pointers) | Type::ConstPtr(ty, pointers) => {
                for _ in 0..*pointers {
                    usize_blob(ELEMENT_TYPE_PTR as usize, blob);
                }
                self.type_blob(ty, blob);
            }
            Type::GenericParam(index) => {
                blob.push(ELEMENT_TYPE_VAR);
                usize_blob(*index as usize, blob);
            }
        }
    }
}

fn round(size: usize, round: usize) -> usize {
    let round = round - 1;
    (size + round) & !round
}

fn usize_blob(value: usize, blob: &mut Vec<u8>) {
    // See II.23.2 in ECMA-335
    assert!(value < 0x20000000);

    if value < 0x80 {
        blob.push(value as u8);
    } else if value < 0x4000 {
        blob.push((0x80 | (value & 0x3F00) >> 8) as u8);
        blob.push((value & 0xFF) as u8);
    } else {
        blob.push((0xC0 | (value & 0x1F000000) >> 24) as u8);
        blob.push(((value & 0xFF0000) >> 16) as u8);
        blob.push(((value & 0xFF00) >> 8) as u8);
        blob.push((value & 0xFF) as u8);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usize_blob() {
        let mut blob = vec![];
        usize_blob(0, &mut blob);
        usize_blob(1, &mut blob);
        usize_blob(2, &mut blob);

        usize_blob(0x80 - 2, &mut blob);
        usize_blob(0x80 - 1, &mut blob);
        usize_blob(0x80, &mut blob);
        usize_blob(0x80 + 1, &mut blob);
        usize_blob(0x80 + 2, &mut blob);

        usize_blob(0x4000 - 2, &mut blob);
        usize_blob(0x4000 - 1, &mut blob);
        usize_blob(0x4000, &mut blob);
        usize_blob(0x4000 + 1, &mut blob);
        usize_blob(0x4000 + 2, &mut blob);

        usize_blob(0x20000000 - 3, &mut blob);
        usize_blob(0x20000000 - 2, &mut blob);
        usize_blob(0x20000000 - 1, &mut blob);

        let mut blob = metadata::Blob::new(0, &blob);
        assert_eq!(blob.read_usize(), 0);
        assert_eq!(blob.read_usize(), 1);
        assert_eq!(blob.read_usize(), 2);

        assert_eq!(blob.read_usize(), 0x80 - 2);
        assert_eq!(blob.read_usize(), 0x80 - 1);
        assert_eq!(blob.read_usize(), 0x80);
        assert_eq!(blob.read_usize(), 0x80 + 1);
        assert_eq!(blob.read_usize(), 0x80 + 2);

        assert_eq!(blob.read_usize(), 0x4000 - 2);
        assert_eq!(blob.read_usize(), 0x4000 - 1);
        assert_eq!(blob.read_usize(), 0x4000);
        assert_eq!(blob.read_usize(), 0x4000 + 1);
        assert_eq!(blob.read_usize(), 0x4000 + 2);

        assert_eq!(blob.read_usize(), 0x20000000 - 3);
        assert_eq!(blob.read_usize(), 0x20000000 - 2);
        assert_eq!(blob.read_usize(), 0x20000000 - 1);

        assert_eq!(blob.slice.len(), 0);
    }
}
