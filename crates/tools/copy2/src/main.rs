use windows_ecma335::*;

fn main() {
    let time = std::time::Instant::now();

    let input = reader::File::read("crates/libs/bindgen/default/Windows.winmd").unwrap();

    let mut output = writer::File::new("test");

    for def in input.table::<reader::TypeDef>() {
        write_def(&mut output, def);
    }

    let bytes = output.into_stream();
    std::fs::write("target/copy.winmd", bytes).unwrap();

    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}

 fn write_attributes<'a, R: reader::HasAttributes<'a> >(output: &mut writer::File, parent: writer::HasAttribute, row: &'a R) {
     for attribute in row.attributes() {
        let ty = attribute.ty();
         let ty = ty.parent();
         let attribute_ref = writer::MemberRefParent::TypeRef(output.TypeRef(ty.namespace(), ty.name()));
//         let args = attribute.args();

//         let mut signature = attribute.ty().signature();
//         let _call_flags = signature.read_u8();
//         let param_count = signature.read_usize();
//         let mut types = vec![];
//         let _return_type = r::Type::from_blob(&mut signature, None, &[]);

//         for _ in 0..param_count {
//             types.push(convert_type(&r::Type::from_blob(&mut signature, None, &[])));
//         }

//         let signature = output.MethodDefSig(&types, &Type::Void, MethodCallAttributes::HASTHIS);
//         let ctor = output.MemberRef(".ctor", signature, attribute_ref);

//         let fixed: Vec<Value> = args
//             .iter()
//             .filter_map(|(name, value)| {
//                 if name.is_empty() {
//                     Some(convert_value(value))
//                 } else {
//                     None
//                 }
//             })
//             .collect();

//         let named: Vec<(&str, Value)> = args
//             .iter()
//             .filter_map(|(name, value)| {
//                 if !name.is_empty() {
//                     Some((*name, convert_value(value)))
//                 } else {
//                     None
//                 }
//             })
//             .collect();

//         let value = output.AttributeValue(&fixed, &named);
//         output.Attribute(parent, w::AttributeType::MemberRef(ctor), value);
     }
 }

fn write_def(output: &mut writer::File, def: reader::TypeDef) {
    let extends = def
        .extends()
        .map(|extends| {
            writer::TypeDefOrRef::TypeRef(output.TypeRef(extends.namespace(), extends.name()))
        })
        .unwrap_or_default();

    let type_def = output.TypeDef(def.namespace(), def.name(), extends, def.flags());

    for field in def.fields() {
        let parent = output.Field(field.name(), &field.ty(), field.flags());

        if let Some(constant) = field.constant() {
            let value = constant.value();
            let ty = value.ty();
            let value = output.ConstantValue(&value);

            output.Constant(writer::HasConstant::Field(parent), ty.code(), value);
        }
    }

    let generics: Vec<_> = def
        .generic_params()
        .map(|param| Type::Generic(param.sequence()))
        .collect();

    // write_attributes(output, w::HasAttribute::TypeDef(type_def), def);

    for interface_impl in def.interface_impls() {
        let interface_impl = output.InterfaceImpl(type_def, &interface_impl.interface(&generics));

        //     write_attributes(
        //         output,
        //         w::HasAttribute::InterfaceImpl(interface_impl),
        //         def_interface,
        //     );
    }

    for generic in def.generic_params() {
        output.GenericParam(
            generic.name(),
            writer::TypeOrMethodDef::TypeDef(type_def),
            generic.sequence(),
            generic.flags(),
        );
    }

    if extends == Default::default() {
        for method in def.methods() {
            let method_def = output.MethodDef(
                method.name(),
                &method.signature(&generics),
                method.flags(),
                method.impl_flags(),
            );

            for param in method.params() {
                output.Param(param.name(), param.sequence(), param.flags());
            }

            //         write_attributes(output, w::HasAttribute::MethodDef(method_def), method);
        }
    }
}
