mod blobs;
mod codes;
mod file;
mod strings;
mod tables;
mod traits;

use super::*;
use blobs::Blobs;
use codes::*;
use strings::Strings;
use traits::*;

struct Gen<'a> {
    blobs: Blobs,
    strings: Strings<'a>,
    tables: tables::Tables,
    module: &'a Module,
    module_scope: u32,
    scopes: HashMap<&'a str, u32>,
    references: HashMap<(&'a str, &'a str), u32>,
}

pub fn write_winmd(module: &Module, path: &str) -> Result<()> {
    let mut gen = Gen::new(module);

    gen.tables.TypeDef.push(tables::TypeDef { TypeName: gen.strings.insert("<Module>"), ..Default::default() });
    gen.module_scope = ResolutionScope::Module(gen.tables.Module.push2(tables::Module { Mvid: 1, ..Default::default() })).encode();

    // Some winmd parsers will fail to read without an `mscorlib` reference. The `insert_module_types` funciton will typically include it
    // automatically but a minimal `Module` tree may not add this dependency.
    gen.insert_scope("System");

    gen.insert_module_types(module);

    let file = file::write(gen.tables.into_stream(), gen.strings.into_stream(), gen.blobs.into_stream());
    write_to_file(path, file)
}

impl<'a> Gen<'a> {
    fn new(module: &'a Module) -> Self {
        Self {
            blobs: Default::default(),
            strings: Default::default(),
            tables: Default::default(),
            module,
            module_scope: 0,
            scopes: Default::default(),
            references: Default::default(),
        }
    }

    fn insert_module_types(&mut self, module: &'a Module) {
        for (name, def) in &module.types {
            self.insert_type_def(&module.namespace, name, def);
        }
        module.modules.values().for_each(|module| self.insert_module_types(module));
    }

    fn insert_type_def(&mut self, namespace: &'a str, name: &'a str, def: &'a [TypeDef]) {
        for def in def {
            let extends = if let Some(extends) = &def.extends { self.insert_type_ref(&extends.namespace, &extends.name) } else { 0 };
            self.tables.TypeDef.push(tables::TypeDef {
                Flags: def.flags.0,
                TypeName: self.strings.insert(name),
                TypeNamespace: self.strings.insert(namespace),
                Extends: extends,
                FieldList: self.tables.Field.len() as _,
                MethodList: self.tables.MethodDef.len() as _,
            });
            for field in &def.fields {
                let blob = self.insert_field_sig(&field.ty);
                let parent = self.tables.Field.push2(tables::Field { Flags: field.flags.0, Name: self.strings.insert(&field.name), Signature: blob });
                if let Some(value) = &field.value {
                    let blob = self.insert_value_blob(value);
                    self.tables.Constant.push(tables::Constant { Type: value.to_code(), Parent: HasConstant::Field(parent).encode(), Value: blob });
                }
            }
            for method in &def.methods {
                let blob = self.insert_method_sig(method);
                self.tables.MethodDef.push(tables::MethodDef { RVA: 0, ImplFlags: 0, Flags: method.flags.0, Name: self.strings.insert(&method.name), Signature: blob, ParamList: self.tables.Param.len() as _ });
                for (sequence, param) in method.params.iter().enumerate() {
                    self.tables.Param.push(tables::Param { Flags: param.flags.0, Sequence: (sequence + 1) as _, Name: self.strings.insert(&param.name) });
                }
            }
        }
    }

    fn insert_value_blob(&mut self, value: &Value) -> u32 {
        // TODO: can either cache in Gen, like we do for scopes and references, or regenerate each time.
        // Profile once we can stress test this with field/method signatures.

        let blob = match value {
            Value::I8(value) => value.to_le_bytes().to_vec(),
            Value::U8(value) => value.to_le_bytes().to_vec(),
            Value::I16(value) => value.to_le_bytes().to_vec(),
            Value::U16(value) => value.to_le_bytes().to_vec(),
            Value::I32(value) => value.to_le_bytes().to_vec(),
            Value::U32(value) => value.to_le_bytes().to_vec(),
            Value::I64(value) => value.to_le_bytes().to_vec(),
            Value::U64(value) => value.to_le_bytes().to_vec(),
            Value::F32(value) => value.to_le_bytes().to_vec(),
            Value::F64(value) => value.to_le_bytes().to_vec(),
            Value::String(value) => {
                let mut blob = vec![];
                u32_blob(value.len() as _, &mut blob);
                blob.extend_from_slice(value.as_bytes());
                blob
            }
            _ => todo!("{:?}", value),
        };

        self.blobs.insert(&blob)
    }

    fn insert_method_sig(&mut self, method: &'a Method) -> u32 {
        // TODO: can either cache in Gen, like we do for scopes and references, or regenerate each time.
        // Profile once we can stress test this with field/method signatures.

        let mut blob = vec![0x20]; // HASTHIS
        u32_blob(method.params.len() as _, &mut blob);
        self.type_blob(&method.return_type.ty, &mut blob);
        for param in &method.params {
            self.type_blob(&param.ty, &mut blob);
        }

        self.blobs.insert(&blob)
    }

    fn insert_field_sig(&mut self, ty: &'a Type) -> u32 {
        // TODO: can either cache in Gen, like we do for scopes and references, or regenerate each time.
        // Profile once we can stress test this with field/method signatures.

        let mut blob = vec![0x6]; // FIELD
        self.type_blob(ty, &mut blob);

        self.blobs.insert(&blob)
    }

    fn insert_scope(&mut self, namespace: &'a str) -> u32 {
        if let Some(scope) = self.scopes.get(namespace) {
            *scope
        } else {
            let name = if namespace == "System" { "mscorlib" } else { namespace };
            let scope = ResolutionScope::AssemblyRef(self.tables.AssemblyRef.push2(tables::AssemblyRef { Name: self.strings.insert(name), ..Default::default() })).encode();
            self.scopes.insert(namespace, scope);
            scope
        }
    }

    fn insert_type_ref(&mut self, namespace: &'a str, name: &'a str) -> u32 {
        if let Some(reference) = self.references.get(&(namespace, name)) {
            *reference
        } else {
            let scope = if self.module.contains_type(namespace, name) { self.module_scope } else { self.insert_scope(namespace) };

            let reference = TypeDefOrRef::TypeRef(self.tables.TypeRef.push2(tables::TypeRef { TypeName: self.strings.insert(name), TypeNamespace: self.strings.insert(namespace), ResolutionScope: scope })).encode();
            self.references.insert((namespace, name), reference);
            reference
        }
    }

    fn type_blob(&mut self, ty: &'a Type, blob: &mut Vec<u8>) {
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
                u32_blob(code, blob);
            }
            Type::HRESULT => {
                let code = self.insert_type_ref("Windows.Foundation", "HResult");
                blob.push(ELEMENT_TYPE_VALUETYPE as _);
                u32_blob(code, blob);
            }
            Type::TypeRef(ty) => {
                let code = self.insert_type_ref(&ty.namespace, &ty.name);
                blob.push(ELEMENT_TYPE_VALUETYPE as _);
                u32_blob(code, blob);
            }
            Type::BSTR => {
                let code = self.insert_type_ref("Windows.Win32.Foundation", "BSTR");
                blob.push(ELEMENT_TYPE_VALUETYPE as _);
                u32_blob(code, blob);
            }
            Type::IUnknown => {
                let code = self.insert_type_ref("Windows.Win32.Foundation", "IUnknown");
                blob.push(ELEMENT_TYPE_VALUETYPE as _);
                u32_blob(code, blob);
            }
            Type::PCWSTR | Type::PWSTR => {
                let code = self.insert_type_ref("Windows.Win32.Foundation", "PWSTR");
                blob.push(ELEMENT_TYPE_VALUETYPE as _);
                u32_blob(code, blob);
            }
            Type::PCSTR | Type::PSTR => {
                let code = self.insert_type_ref("Windows.Win32.Foundation", "PSTR");
                blob.push(ELEMENT_TYPE_VALUETYPE as _);
                u32_blob(code, blob);
            }
            Type::ConstRef(ty) => {
                u32_blob(ELEMENT_TYPE_CMOD_OPT as _, blob);
                u32_blob(self.insert_type_ref("System.Runtime.CompilerServices", "IsConst"), blob);
                u32_blob(ELEMENT_TYPE_BYREF as _, blob);
                self.type_blob(ty, blob);
            }
            Type::WinrtArrayRef(ty) => {
                u32_blob(ELEMENT_TYPE_BYREF as _, blob);
                u32_blob(ELEMENT_TYPE_SZARRAY as _, blob);
                self.type_blob(ty, blob);
            }
            Type::WinrtArray(ty) => {
                u32_blob(ELEMENT_TYPE_SZARRAY as _, blob);
                self.type_blob(ty, blob);
            }
            Type::Win32Array((ty, bounds)) => {
                u32_blob(ELEMENT_TYPE_ARRAY as _, blob);
                self.type_blob(ty, blob);
                u32_blob(1, blob); // rank
                u32_blob(1, blob); // count
                u32_blob(*bounds as _, blob);
            }
            Type::TypeName => {
                let code = self.insert_type_ref("System", "Type");
                blob.push(ELEMENT_TYPE_CLASS as _);
                u32_blob(code, blob);
            }
            Type::MutPtr((ty, pointers)) | Type::ConstPtr((ty, pointers)) => {
                for _ in 0..*pointers {
                    u32_blob(ELEMENT_TYPE_PTR as _, blob);
                }
                self.type_blob(ty, blob);
            }
            _ => todo!("{:?}", ty),
        }
    }
}

fn round(size: usize, round: usize) -> usize {
    let round = round - 1;
    (size + round) & !round
}

fn u32_blob(value: u32, blob: &mut Vec<u8>) {
    if value < 0x80 {
        blob.push(value as _);
    } else if value < 0x4000 {
        blob.push((0x40 | (value & 0xFF00)) as _);
        blob.push((value | 0xFF) as _);
    } else {
        blob.push((0x60 | (value & 0xFF000000)) as _);
        blob.push((value | 0xFF0000) as _);
        blob.push((value | 0xFF00) as _);
        blob.push((value | 0xFF) as _);
    }
}
