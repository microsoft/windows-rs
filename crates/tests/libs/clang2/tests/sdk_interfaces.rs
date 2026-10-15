use windows_clang2::*;
use windows_metadata::{Type as MetadataType, Value, reader, reader::HasAttributes};

const ROOTS: &[&str] = &[
    "IStream",
    "IWICImagingFactory",
    "IPropertyStore",
    "IStorage",
    "IPersistStream",
    "IMoniker",
    "IBindCtx",
    "IRunningObjectTable",
    "IPropertyStorage",
    "IFileOpenDialog",
    "IFileSaveDialog",
    "IFileOperation",
    "ID3D11Device",
    "ID3D11DeviceContext",
    "IDXGISwapChain4",
    "ID3D12Device",
    "ID3D12GraphicsCommandList",
    "IDWriteLocalFontFileLoader",
    "IDWriteTextAnalysisSource",
    "IDWriteTextAnalysisSink",
    "IDWritePixelSnapping",
    "IDWriteRenderingParams",
    "ID2D1SimplifiedGeometrySink",
    "IDWriteFactory",
    "IDWriteTextAnalyzer",
    "ID2D1Factory4",
    "IWICImagingFactory2",
    "IWICDevelopRaw",
    "IDWriteFactory7",
    "IDWriteFontSet3",
    "ID2D1Factory7",
    "ID2D1DeviceContext6",
];

fn check_deprecations<'a>(
    native: &[Attribute],
    projected: impl Iterator<Item = reader::Attribute<'a>>,
) {
    let mut expected: Vec<_> = native
        .iter()
        .filter_map(|attribute| {
            if let AttributeKind::Deprecated(value) = &attribute.kind {
                Some((value.message.clone(), value.replacement.clone()))
            } else {
                None
            }
        })
        .collect();
    expected.sort();
    expected.dedup();
    let mut actual: Vec<_> = projected
        .filter(|attribute| attribute.name() == "NativeDeprecatedAttribute")
        .map(|attribute| {
            let values = attribute.value();
            let (Value::Utf8(message), Value::Utf8(replacement)) = (&values[0].1, &values[1].1)
            else {
                panic!()
            };
            (message.clone(), replacement.clone())
        })
        .collect();
    actual.sort();
    assert_eq!(actual, expected);
}

fn input(roots: &[&str]) -> Input {
    let mut input = test_clang2::annotated_sdk_fixture();
    input.name = "sdk_interfaces.h".into();
    input.source = test_clang2::fixture("sdk_interfaces").source;
    input.roots = roots.iter().map(|name| (*name).into()).collect();
    input
}

#[test]
#[ignore = "requires the pinned Windows SDK and clang resource headers"]
fn sdk_interfaces() {
    helpers::ensure_libclang();
    let input = input(ROOTS);
    let targets = [Target::X86, Target::X64, Target::Arm64];
    let snapshots = capture(std::slice::from_ref(&input), &targets).unwrap();
    let output = projection::project(&merge(&snapshots).unwrap(), "Fixture").unwrap();
    assert!(output.interfaces >= ROOTS.len());
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang2-sdk-interfaces.winmd");
    let rdl = winmd.with_extension("rdl");
    let again = winmd.with_extension("again.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_texts(output.partitions.values().map(|part| &part.rdl))
        .output(&winmd)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rdl)
        .write()
        .unwrap();
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input(&rdl)
        .output(&again)
        .write()
        .unwrap();
    for path in [&winmd, &again] {
        for (snapshot, bit) in snapshots.iter().zip([1, 2, 4]) {
            let index =
                reader::Index::new_for_architecture(vec![reader::File::read(path).unwrap()], bit);
            let mut interfaces = 0;
            for record in &snapshot.records {
                let Some((native, uuid)) = record.observations.iter().find_map(|id| {
                    let observation = &snapshot.observations[id.0];
                    let native = observation.definition.as_ref()?;
                    let uuid =
                        observation.attributes.iter().find_map(|attribute| {
                            match attribute.kind {
                                AttributeKind::Uuid(uuid) => Some(uuid),
                                _ => None,
                            }
                        })?;
                    (!native.methods.is_empty() || !native.bases.is_empty())
                        .then_some((native, uuid))
                }) else {
                    continue;
                };
                interfaces += 1;
                let name = record.name.as_str();
                let projected = index.expect("Fixture", name);
                let observation = record
                    .observations
                    .iter()
                    .map(|id| &snapshot.observations[id.0])
                    .find(|observation| observation.definition.is_some())
                    .unwrap();
                check_deprecations(&observation.attributes, projected.attributes());
                assert_eq!(
                    projected
                        .methods()
                        .map(|method| method.name())
                        .collect::<Vec<_>>(),
                    native
                        .methods
                        .iter()
                        .map(|method| method.name.as_str())
                        .collect::<Vec<_>>(),
                    "{name}",
                );
                assert_eq!(
                    projected
                        .interface_impls()
                        .map(|base| base.interface(&[]))
                        .collect::<Vec<_>>(),
                    native
                        .bases
                        .iter()
                        .map(|base| MetadataType::class_named(
                            "Fixture",
                            &snapshot.records[base.record.0].name
                        ))
                        .collect::<Vec<_>>(),
                    "{name}",
                );
                let mut expected = vec![
                    Value::U32((uuid >> 96) as u32),
                    Value::U16((uuid >> 80) as u16),
                    Value::U16((uuid >> 64) as u16),
                ];
                expected.extend((uuid as u64).to_be_bytes().map(Value::U8));
                assert_eq!(
                    projected
                        .find_attribute("GuidAttribute")
                        .unwrap()
                        .value()
                        .into_iter()
                        .map(|(_, value)| value)
                        .collect::<Vec<_>>(),
                    expected,
                    "{name}",
                );
                for (method, native) in projected.methods().zip(&native.methods) {
                    check_deprecations(&native.observation.attributes, method.attributes());
                    let mut id = native.observation.written;
                    let function = loop {
                        match &snapshot.types[id.0].kind {
                            TypeKind::Attributed { modified, .. } => id = *modified,
                            TypeKind::Function(function) => break function,
                            _ => panic!(),
                        }
                    };
                    let expected = match function.exceptions {
                        ExceptionSpecification::None => None,
                        ExceptionSpecification::Noexcept => Some("noexcept"),
                        ExceptionSpecification::NoThrow => Some("nothrow"),
                        ExceptionSpecification::DynamicNone => Some("throw()"),
                    };
                    assert_eq!(
                        method
                            .find_attribute("NativeExceptionSpecificationAttribute")
                            .map(|attribute| attribute
                                .value()
                                .into_iter()
                                .map(|(_, value)| value)
                                .collect::<Vec<_>>()),
                        expected.map(|value| vec![Value::Utf8(value.into())]),
                        "{name}.{}",
                        method.name(),
                    );
                    let ValueKind::Function { parameters, .. } = &native.observation.kind else {
                        panic!()
                    };
                    assert_eq!(
                        method
                            .find_attribute("NativeConstMethodAttribute")
                            .is_some(),
                        function.method_qualifiers.constant,
                        "{name}.{}",
                        method.name(),
                    );
                    for (parameter, native) in method.params().zip(parameters) {
                        assert_eq!(
                            parameter
                                .find_attribute("NativeDefaultArgumentAttribute")
                                .map(|attribute| attribute.value()[0].1.clone()),
                            native
                                .default
                                .as_ref()
                                .map(|value| Value::Utf8(value.expression.clone())),
                            "{name}.{}({})",
                            method.name(),
                            parameter.name(),
                        );
                    }
                    assert_eq!(
                        method
                            .params()
                            .map(|parameter| parameter.name().to_string())
                            .collect::<Vec<_>>(),
                        parameters
                            .iter()
                            .map(|parameter| parameter.name.as_str())
                            .collect::<Vec<_>>(),
                        "{name}.{}",
                        method.name(),
                    );
                    assert_eq!(method.signature(&[]).types.len(), parameters.len());
                    assert!(
                        method
                            .signature(&[])
                            .flags
                            .contains(windows_metadata::MethodCallAttributes::HASTHIS)
                    );
                }
            }
            assert_eq!(interfaces, output.interfaces);
            let factory = index.expect("Fixture", "IWICImagingFactory");
            let stream = factory
                .methods()
                .find(|method| method.name() == "CreateDecoderFromStream")
                .unwrap();
            assert_eq!(
                stream.signature(&[]).types[0],
                MetadataType::class_named("Fixture", "IStream")
            );
            for name in ["ID3D11Device", "IDXGIObject", "ID3D12Object"] {
                let interface = index.expect("Fixture", name);
                let method = interface
                    .methods()
                    .find(|method| method.name() == "SetPrivateDataInterface")
                    .unwrap();
                assert_eq!(
                    method.signature(&[]).types[1],
                    MetadataType::class_named("Fixture", "IUnknown")
                );
            }
            let context = index.expect("Fixture", "ID3D11DeviceContext");
            let method = context
                .methods()
                .find(|method| method.name() == "VSSetConstantBuffers")
                .unwrap();
            assert_eq!(
                method.signature(&[]).types[2],
                MetadataType::PtrConst(
                    Box::new(MetadataType::class_named("Fixture", "ID3D11Buffer")),
                    1
                )
            );
        }
    }
    for path in [winmd, rdl, again] {
        std::fs::remove_file(path).unwrap();
    }
    let partial = projection::generate_partial(&[input], "Fixture").unwrap();
    assert!(partial.omissions.is_empty(), "{:?}", partial.omissions);
    assert_eq!(partial.output, output);
}
