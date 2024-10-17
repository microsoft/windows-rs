use super::*;

impl Type {
    pub fn runtime_signature(&self) -> String {
        match self {
            Self::Bool => "b1".to_string(),
            Self::Char => "c2".to_string(),
            Self::I8 => "i1".to_string(),
            Self::U8 => "u1".to_string(),
            Self::I16 => "i2".to_string(),
            Self::U16 => "u2".to_string(),
            Self::I32 => "i4".to_string(),
            Self::U32 => "u4".to_string(),
            Self::I64 => "i8".to_string(),
            Self::U64 => "u8".to_string(),
            Self::F32 => "f4".to_string(),
            Self::F64 => "f8".to_string(),
            Self::ISize => "is".to_string(),
            Self::USize => "us".to_string(),
            Self::String => "string".to_string(),
            Self::Object => "cinterface(IInspectable)".to_string(),
            Self::GUID => "g16".to_string(),
            Self::HRESULT => "struct(Windows.Foundation.HResult;i4)".to_string(),
            Self::Item(item) => item.runtime_signature(),
            rest => panic!("windows-bindgen: {rest:?}"),
        }
    }
}

// TODO: put signatures in their own rs file?
impl Item {
    pub fn runtime_signature(&self) -> String {
        match self {
            Self::Class(item) => item.runtime_signature(),
            Self::Delegate(item) => item.runtime_signature(),
            Self::Enum(item) => item.runtime_signature(),
            Self::Interface(item) => item.runtime_signature(),
            Self::Struct(item) => item.runtime_signature(),
            rest => panic!("windows-bindgen: {rest:?}"),
        }
    }
}

impl Class {
    pub fn runtime_signature(&self) -> String {
        format!(
            "rc({}.{};{})",
            self.def.namespace(),
            self.def.name(),
            self.default_interface(&self.generics)
                .unwrap()
                .runtime_signature()
        )
    }
}

impl Interface {
    pub fn runtime_signature(&self) -> String {
        interface_signature(self.def, &self.generics)
    }
}

impl Delegate {
    pub fn runtime_signature(&self) -> String {
        if self.generics.is_empty() {
            let guid = self.def.guid_attribute().unwrap();
            format!("delegate({{{guid}}})")
        } else {
            interface_signature(self.def, &self.generics)
        }
    }
}

impl Struct {
    pub fn runtime_signature(&self) -> String {
        let mut signature = format!("struct({}.{}", self.def.namespace(), self.def.name());
        for field in self.def.fields() {
            signature.push(';');
            signature.push_str(&field.ty(None).runtime_signature());
        }
        signature.push(')');
        signature
    }
}

impl Enum {
    pub fn runtime_signature(&self) -> String {
        format!(
            "enum({}.{};{})",
            self.def.namespace(),
            self.def.name(),
            self.def.underlying_type().runtime_signature()
        )
    }
}

fn interface_signature(def: TypeDef, generics: &[Type]) -> String {
    if generics.is_empty() {
        let guid = def.guid_attribute().unwrap();
        format!("{{{guid}}}")
    } else {
        let guid = def.guid_attribute().unwrap();
        let mut signature = format!("pinterface({{{guid}}}");

        for generic in generics {
            signature.push(';');
            signature.push_str(&generic.runtime_signature())
        }

        signature.push(')');
        signature
    }
}
