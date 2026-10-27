use windows_metadata::{reader::*, writer, *};

fn pointer(constant: bool, ty: Type) -> Type {
    match (constant, ty) {
        (true, Type::PtrConst(inner, count)) => Type::PtrConst(inner, count + 1),
        (false, Type::PtrMut(inner, count)) => Type::PtrMut(inner, count + 1),
        (true, inner) => Type::PtrConst(Box::new(inner), 1),
        (false, inner) => Type::PtrMut(Box::new(inner), 1),
    }
}

fn encode(ty: &Type) -> Index {
    let mut file = writer::File::new("pointers");
    let base = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "ValueType"));
    file.TypeDef("Test", "Pointers", base, TypeAttributes::Public);
    file.Field("value", ty, FieldAttributes::Public);
    Index::new(vec![File::new(file.into_stream()).unwrap()])
}

#[test]
fn pointer_constness() {
    for depth in 1..=5 {
        for mask in 0..1 << depth {
            let levels: Vec<_> = (0..depth).map(|bit| mask & (1 << bit) != 0).collect();
            let ty = levels
                .iter()
                .rev()
                .fold(Type::I32, |ty, &constant| pointer(constant, ty));
            let index = encode(&ty);
            let field = index.expect("Test", "Pointers").fields().next().unwrap();
            assert_eq!(field.ty(), ty);

            let mixed = levels.iter().any(|&constant| constant != levels[0]);
            let mut expected = vec![0x06];
            if !mixed && levels[0] {
                expected.push(0x1f);
            }

            for constant in levels {
                expected.push(0x0f);
                if mixed && constant {
                    expected.push(0x1f);
                }
            }
            expected.push(0x08);
            let mut blob = field.blob(2);
            let mut actual = Vec::new();
            while !blob.is_empty() {
                let code = blob.read_u8();
                actual.push(code);
                if code == 0x1f {
                    let modifier = blob.decode::<TypeDefOrRef>();
                    assert_eq!(modifier.namespace(), "System.Runtime.CompilerServices");
                    assert_eq!(modifier.name(), "IsConst");
                }
            }
            assert_eq!(actual, expected, "{ty:?}");

            for wrapped in [
                Type::ArrayFixed(Box::new(ty.clone()), 3),
                Type::RefMut(Box::new(ty.clone())),
                Type::RefConst(Box::new(ty.clone())),
                pointer(false, Type::ArrayFixed(Box::new(ty), 3)),
            ] {
                let index = encode(&wrapped);
                assert_eq!(
                    index
                        .expect("Test", "Pointers")
                        .fields()
                        .next()
                        .unwrap()
                        .ty(),
                    wrapped
                );
            }
        }
    }
}
