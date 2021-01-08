use crate::*;

#[derive(Debug)]
pub struct NativeMethod {
    pub def: winmd::MethodDef,
    pub params: Vec<(&'static str, Type)>,
    pub return_type: Option<Type>,
}

impl NativeMethod {
    pub fn new(method: &winmd::MethodDef, calling_namespace: &'static str,) -> Self {
        let mut blob = method.sig();

        if blob.read_unsigned() & 0x10 != 0 {
            blob.read_unsigned();
        }

        let param_count = blob.read_unsigned();
        blob.read_modifiers();
        blob.read_expected(0x10);

        let mut params = Vec::with_capacity(param_count as usize);

        let return_type = if blob.read_expected(0x01) {
            None
        } else {
            Some(Type::from_blob(&mut blob, &[], calling_namespace))
        };

        for param in method.params() {
            if return_type.is_none() || param.sequence() != 0 {
                blob.read_modifiers(); // const
                blob.read_expected(0x10); // ref
                params.push((
                    param.name(),
                    Type::from_blob(&mut blob, &[], calling_namespace),
                ));
            }
        }

        Self {
            def: *method,
            params,
            return_type,
        }
    }
}
