mod blobs;
mod codes;
mod file;
mod strings;
mod tables;
mod traits;
mod r#type;

use blobs::Blobs;
use codes::*;
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
    pub references: HashMap<String, HashMap<String, u32>>,
}

impl Writer {
    pub fn new(name: &str) -> Self {
        let mut writer = Self {
            blobs: Default::default(),
            strings: Default::default(),
            tables: Default::default(),
            scopes: Default::default(),
            references: Default::default(),
        };

        writer.tables.TypeDef.push(TypeDef {
            TypeName: writer.strings.insert("<Module>"),
            ..Default::default()
        });

        let name = name
            .rsplit_once(&['/', '\\'])
            .map_or(name, |(_, name)| name);

        writer.tables.Module.push(Module {
            Name: writer.strings.insert(name),
            Mvid: 1,
            ..Default::default()
        });

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
        file::write(
            self.tables.into_stream(),
            self.strings.into_stream(),
            self.blobs.into_stream(),
        )
    }

    // fn insert_module_types(&mut self, module: &'a Module) {
    //     for (name, def) in &module.types {
    //         self.insert_type_def(&module.namespace, name, def);
    //     }
    //     module.modules.values().for_each(|module| self.insert_module_types(module));
    // }

    // fn insert_type_def(&mut self, namespace: &'a str, name: &'a str, def: &'a [TypeDef]) {
    //     for def in def {
    //         let extends = if let Some(extends) = &def.extends { self.insert_type_ref(&extends.namespace, &extends.name) } else { 0 };
    //         self.tables.TypeDef.push(TypeDef {
    //             Flags: def.flags.0,
    //             TypeName: self.strings.insert(name),
    //             TypeNamespace: self.strings.insert(namespace),
    //             Extends: extends,
    //             FieldList: self.tables.Field.len() as _,
    //             MethodList: self.tables.MethodDef.len() as _,
    //         });
    //         for field in &def.fields {
    //             let blob = self.insert_field_sig(&field.ty);
    //             let parent = self.tables.Field.push2(Field { Flags: field.flags.0, Name: self.strings.insert(&field.name), Signature: blob });
    //             if let Some(value) = &field.value {
    //                 let blob = self.insert_value_blob(value);
    //                 self.tables.Constant.push(Constant { Type: value.to_code(), Parent: HasConstant::Field(parent).encode(), Value: blob });
    //             }
    //         }
    //         for method in &def.methods {
    //             let blob = self.insert_method_sig(method);
    //             self.tables.MethodDef.push(MethodDef { RVA: 0, ImplFlags: 0, Flags: method.flags.0, Name: self.strings.insert(&method.name), Signature: blob, ParamList: self.tables.Param.len() as _ });
    //             for (sequence, param) in method.params.iter().enumerate() {
    //                 self.tables.Param.push(Param { Flags: param.flags.0, Sequence: (sequence + 1) as _, Name: self.strings.insert(&param.name) });
    //             }
    //         }
    //     }
    // }

    // fn insert_value_blob(&mut self, value: &Value) -> u32 {
    //     // TODO: can either cache in Writer, like we do for scopes and references, or regenerate each time.
    //     // Profile once we can stress test this with field/method signatures.

    //     let blob = match value {
    //         Value::I8(value) => value.to_le_bytes().to_vec(),
    //         Value::U8(value) => value.to_le_bytes().to_vec(),
    //         Value::I16(value) => value.to_le_bytes().to_vec(),
    //         Value::U16(value) => value.to_le_bytes().to_vec(),
    //         Value::I32(value) => value.to_le_bytes().to_vec(),
    //         Value::U32(value) => value.to_le_bytes().to_vec(),
    //         Value::I64(value) => value.to_le_bytes().to_vec(),
    //         Value::U64(value) => value.to_le_bytes().to_vec(),
    //         Value::F32(value) => value.to_le_bytes().to_vec(),
    //         Value::F64(value) => value.to_le_bytes().to_vec(),
    //         Value::String(value) => {
    //             let mut blob = vec![];
    //             usize_blob(value.len(), &mut blob);
    //             blob.extend_from_slice(value.as_bytes());
    //             blob
    //         }
    //         rest => unimplemented!("{rest:?}"),
    //     };

    //     self.blobs.insert(&blob)
    // }

    // fn insert_method_sig(&mut self, method: &'a Method) -> u32 {
    //     // TODO: can either cache in Writer, like we do for scopes and references, or regenerate each time.
    //     // Profile once we can stress test this with field/method signatures.

    //     let mut blob = vec![method.call_flags.0];
    //     usize_blob(method.params.len(), &mut blob);
    //     self.type_blob(&method.return_type.ty, &mut blob);
    //     for param in &method.params {
    //         self.type_blob(&param.ty, &mut blob);
    //     }

    //     self.blobs.insert(&blob)
    // }

    pub fn insert_field_sig(&mut self, ty: &Type) -> u32 {
        // TODO: can either cache in Writer, like we do for scopes and references, or regenerate each time.
        // Profile once we can stress test this with field/method signatures.

        let mut blob = vec![0x6]; // FIELD
        self.type_blob(ty, &mut blob);

        self.blobs.insert(&blob)
    }

    fn insert_scope(&mut self, namespace: &str) -> u32 {
        if let Some(scope) = self.scopes.get(namespace) {
            *scope
        } else if namespace == "System" {
            let scope = ResolutionScope::AssemblyRef(
                self.tables.AssemblyRef.push2(AssemblyRef {
                    Name: self.strings.insert("mscorlib"),
                    MajorVersion: 4,
                    PublicKeyOrToken: self
                        .blobs
                        .insert(&[0xB7, 0x7A, 0x5C, 0x56, 0x19, 0x34, 0xE0, 0x89]),
                    ..Default::default()
                }),
            )
            .encode();
            self.scopes.insert(namespace.to_string(), scope);
            scope
        } else {
            // TODO: may need to capture the original assembly info for external references.
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
        if let Some(key) = self.references.get(namespace) {
            if let Some(reference) = key.get(name) {
                return *reference;
            }
        }

        let scope = self.insert_scope(namespace);

        let reference = TypeDefOrRef::TypeRef(self.tables.TypeRef.push2(TypeRef {
            TypeName: self.strings.insert(name),
            TypeNamespace: self.strings.insert(namespace),
            ResolutionScope: scope,
        }))
        .encode();
        self.references
            .entry(namespace.to_string())
            .or_default()
            .insert(name.to_string(), reference);
        reference
    }

    fn type_blob(&mut self, ty: &Type, blob: &mut Vec<u8>) {
        match ty {
            Type::Void => blob.push(ELEMENT_TYPE_VOID as _),
            Type::Bool => blob.push(ELEMENT_TYPE_BOOLEAN as _),
            Type::Char => blob.push(ELEMENT_TYPE_CHAR as _),
            Type::I8 => blob.push(ELEMENT_TYPE_I1 as _),
            Type::U8 => blob.push(ELEMENT_TYPE_U1 as _),
            Type::I16 => blob.push(ELEMENT_TYPE_I2 as _),
            Type::U16 => blob.push(ELEMENT_TYPE_U2 as _),
            Type::I32 => blob.push(ELEMENT_TYPE_I4 as _),
            Type::U32 => blob.push(ELEMENT_TYPE_U4 as _),
            Type::I64 => blob.push(ELEMENT_TYPE_I8 as _),
            Type::U64 => blob.push(ELEMENT_TYPE_U8 as _),
            Type::F32 => blob.push(ELEMENT_TYPE_R4 as _),
            Type::F64 => blob.push(ELEMENT_TYPE_R8 as _),
            Type::ISize => blob.push(ELEMENT_TYPE_I as _),
            Type::USize => blob.push(ELEMENT_TYPE_U as _),
            Type::String => blob.push(ELEMENT_TYPE_STRING as _),
            Type::IInspectable => blob.push(ELEMENT_TYPE_OBJECT as _),
            Type::GUID => {
                let code = self.insert_type_ref("System", "Guid");
                blob.push(ELEMENT_TYPE_VALUETYPE as _);
                usize_blob(code as _, blob);
            }
            Type::HRESULT => {
                let code = self.insert_type_ref("Windows.Foundation", "HResult");
                blob.push(ELEMENT_TYPE_VALUETYPE as _);
                usize_blob(code as _, blob);
            }
            Type::TypeRef(ty) => {
                let code = self.insert_type_ref(&ty.namespace, &ty.name);
                blob.push(ELEMENT_TYPE_VALUETYPE as _);
                usize_blob(code as _, blob);
            }
            Type::BSTR => {
                let code = self.insert_type_ref("Windows.Win32.Foundation", "BSTR");
                blob.push(ELEMENT_TYPE_VALUETYPE as _);
                usize_blob(code as _, blob);
            }
            Type::IUnknown => {
                let code = self.insert_type_ref("Windows.Win32.Foundation", "IUnknown");
                blob.push(ELEMENT_TYPE_VALUETYPE as _);
                usize_blob(code as _, blob);
            }
            Type::PCWSTR | Type::PWSTR => {
                let code = self.insert_type_ref("Windows.Win32.Foundation", "PWSTR");
                blob.push(ELEMENT_TYPE_VALUETYPE as _);
                usize_blob(code as _, blob);
            }
            Type::PCSTR | Type::PSTR => {
                let code = self.insert_type_ref("Windows.Win32.Foundation", "PSTR");
                blob.push(ELEMENT_TYPE_VALUETYPE as _);
                usize_blob(code as _, blob);
            }
            Type::ConstRef(ty) => {
                usize_blob(ELEMENT_TYPE_CMOD_OPT as _, blob);
                usize_blob(
                    self.insert_type_ref("System.Runtime.CompilerServices", "IsConst") as _,
                    blob,
                );
                usize_blob(ELEMENT_TYPE_BYREF as _, blob);
                self.type_blob(ty, blob);
            }
            Type::WinrtArrayRef(ty) => {
                usize_blob(ELEMENT_TYPE_BYREF as _, blob);
                usize_blob(ELEMENT_TYPE_SZARRAY as _, blob);
                self.type_blob(ty, blob);
            }
            Type::WinrtArray(ty) => {
                usize_blob(ELEMENT_TYPE_SZARRAY as _, blob);
                self.type_blob(ty, blob);
            }
            Type::Win32Array(ty, bounds) => {
                usize_blob(ELEMENT_TYPE_ARRAY as _, blob);
                self.type_blob(ty, blob);
                usize_blob(1, blob); // rank
                usize_blob(1, blob); // count
                usize_blob(*bounds as _, blob);
            }
            Type::TypeName => {
                let code = self.insert_type_ref("System", "Type");
                blob.push(ELEMENT_TYPE_CLASS as _);
                usize_blob(code as _, blob);
            }
            Type::MutPtr(ty, pointers) | Type::ConstPtr(ty, pointers) => {
                for _ in 0..*pointers {
                    usize_blob(ELEMENT_TYPE_PTR as _, blob);
                }
                self.type_blob(ty, blob);
            }
            rest => unimplemented!("{rest:?}"),
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
        blob.push(value as _);
    } else if value < 0x4000 {
        blob.push((0x80 | (value & 0x3F00) >> 8) as _);
        blob.push((value & 0xFF) as _);
    } else {
        blob.push((0xC0 | (value & 0x1F000000) >> 24) as _);
        blob.push(((value & 0xFF0000) >> 16) as _);
        blob.push(((value & 0xFF00) >> 8) as _);
        blob.push((value & 0xFF) as _);
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
