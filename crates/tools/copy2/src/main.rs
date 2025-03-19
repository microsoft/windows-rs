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

// fn write_attributes<R: r::HasAttributes>(output: &mut w::File, parent: w::HasAttribute, row: R) {
//     for attribute in row.attributes() {
//         let ty = attribute.ty().parent();
//         let attribute_ref = w::MemberRefParent::TypeRef(output.TypeRef(ty.namespace(), ty.name()));
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
//     }
// }

fn write_def(output: &mut writer::File, def: reader::TypeDef) {
    let extends = def
        .extends()
        .map(|extends| {
            writer::TypeDefOrRef::TypeRef(output.TypeRef(extends.namespace(), extends.name()))
        })
        .unwrap_or_default();

    let type_def = output.TypeDef(def.namespace(), def.name(), extends, def.flags());

    for field in def.fields() {
        let signature = output.FieldSig(&field.ty());

        let parent = output.Field(field.name(), signature, field.flags());

        if let Some(constant) = field.constant() {
            let value = &constant.value();
            let ty = value.ty();
            let value = output.ConstantValue(&value);

            output.Constant(writer::HasConstant::Field(parent), ty.code(), value);
        }
    }

    // write_attributes(output, w::HasAttribute::TypeDef(type_def), def);

    // let generics = def.generics();

    // for def_interface in def.interface_impls() {
    //     let interface = convert_type(&def_interface.ty(&generics));

    //     let interface = if let Type::Name(tn) = interface {
    //         if tn.generics.is_empty() {
    //             w::TypeDefOrRef::TypeRef(output.TypeRef(&tn.namespace, &tn.name))
    //         } else {
    //             w::TypeDefOrRef::TypeSpec(output.TypeSpec(&tn.namespace, &tn.name, &tn.generics))
    //         }
    //     } else {
    //         panic!();
    //     };

    //     let interface_impl = output.InterfaceImpl(type_def, interface);

    //     write_attributes(
    //         output,
    //         w::HasAttribute::InterfaceImpl(interface_impl),
    //         def_interface,
    //     );
    // }

    // for generic in def.generic_params() {
    //     output.GenericParam(
    //         generic.name(),
    //         w::TypeOrMethodDef::TypeDef(type_def),
    //         generic.sequence(),
    //         generic.flags(),
    //     );
    // }

    // // Methods on classes is a huge overhead on .winmd size but adds no value as all of this information
    // // is redundantly stored elsewhere.
    // if include_methods {
    //     for method in def.methods() {
    //         let signature = method.signature("", &generics);
    //         let types: Vec<Type> = signature
    //             .params
    //             .iter()
    //             .map(|param| convert_type(&param.ty))
    //             .collect();
    //         let signature_blob = output.MethodDefSig(
    //             &types,
    //             &convert_type(&signature.return_type),
    //             MethodCallAttributes(signature.call_flags.0),
    //         );
    //         let flags = MethodAttributes(method.flags().0);
    //         let impl_flags = MethodImplAttributes(method.impl_flags().0);

    //         let method_def = output.MethodDef(method.name(), signature_blob, flags, impl_flags);

    //         if let Some(return_param) = signature.return_param {
    //             output.Param(
    //                 return_param.name(),
    //                 return_param.sequence(),
    //                 ParamAttributes(return_param.flags().0),
    //             );
    //         }

    //         for param in signature.params {
    //             output.Param(
    //                 param.def.name(),
    //                 param.def.sequence(),
    //                 ParamAttributes(param.def.flags().0),
    //             );
    //         }

    //         write_attributes(output, w::HasAttribute::MethodDef(method_def), method);
    //     }
    // }
}
