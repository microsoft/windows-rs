use crate::*;

#[derive(Debug)]
pub struct NativeMethod {
    pub def: winmd::MethodDef,
    pub params: Vec<NativeParam>,
    pub return_type: Option<Type>,
}

#[derive(Debug)]
pub struct NativeParam {
    pub name: &'static str,
    pub t: Type,
    pub is_const: bool,
}

impl NativeMethod {
    pub fn new(method: &winmd::MethodDef, calling_namespace: &'static str) -> Self {
        let mut blob = method.sig();

        if blob.read_unsigned() & 0x10 != 0 {
            panic!();
            blob.read_unsigned(); // generic param count
        }

        let param_count = blob.read_unsigned();
        blob.read_modifiers();
        blob.read_expected(0x10); // byref

        let mut params = Vec::with_capacity(param_count as usize);

        let return_type = if blob.read_expected(0x01) {
            None
        } else {
            Some(Type::from_blob(&mut blob, &[], calling_namespace))
        };

        for param in method.params() {
            if return_type.is_none() || param.sequence() != 0 {
                let mods = blob.read_modifiers();
                assert!(mods.len() == 0);
                let is_const = mods
                    .iter()
                    .any(|def| def.name() == ("Windows.Win32.Interop", "ConstAttribute"));
                assert!(!is_const);

                blob.read_expected(0x10); // byref
                let t = Type::from_blob(&mut blob, &[], calling_namespace);
                let name = param.name();

                params.push(NativeParam { name, t, is_const });
            }
        }

        Self {
            def: *method,
            params,
            return_type,
        }
    }

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        let mut defs = Vec::new();

        if let Some(t) = &self.return_type {
            defs.append(&mut t.kind.dependencies());
        }

        for param in &self.params {
            defs.append(&mut param.t.kind.dependencies());
        }

        defs
    }
}
