use windows_clang2::{FactKind, Input, extract};

#[test]
fn facts_are_tu_local_and_input_order_independent() {
    windows_clang::ensure_libclang();

    let alpha = Input::new(
        "alpha.hpp",
        "#define ALPHA 1\n\
         typedef unsigned short Status;\n\
         enum State { Ready = 1 };\n\
         struct Outer { struct Inner { int value; } inner; };\n\
         #define MAKE_GENERATED typedef int Generated;\n\
         MAKE_GENERATED\n\
         namespace nested { MAKE_GENERATED }\n",
    );
    let beta = Input::new(
        "beta.hpp",
        "#define BETA 2\n\
         typedef unsigned int Status;\n\
         struct Payload { int value; };\n\
         int run(Payload* value);\n",
    );

    let forward = extract([alpha.clone(), beta.clone()], &["-x", "c++"]).unwrap();
    let reverse = extract([beta.clone(), alpha.clone()], &["-x", "c++"]).unwrap();

    assert_eq!(forward, reverse);
    assert_eq!(forward.dump(), reverse.dump());
    assert_eq!(
        forward
            .facts()
            .iter()
            .map(|fact| (fact.origin.tu.as_str(), fact.kind, fact.name.as_str()))
            .collect::<Vec<_>>(),
        [
            ("alpha.hpp", FactKind::Macro, "ALPHA"),
            ("alpha.hpp", FactKind::Macro, "MAKE_GENERATED"),
            ("alpha.hpp", FactKind::Typedef, "Status"),
            ("alpha.hpp", FactKind::Enum, "State"),
            ("alpha.hpp", FactKind::Struct, "Outer"),
            ("alpha.hpp", FactKind::Struct, "Inner"),
            ("alpha.hpp", FactKind::Typedef, "Generated"),
            ("alpha.hpp", FactKind::Namespace, "nested"),
            ("alpha.hpp", FactKind::Typedef, "Generated"),
            ("beta.hpp", FactKind::Macro, "BETA"),
            ("beta.hpp", FactKind::Typedef, "Status"),
            ("beta.hpp", FactKind::Struct, "Payload"),
            ("beta.hpp", FactKind::Function, "run"),
        ]
    );

    let inner = forward
        .facts()
        .iter()
        .find(|fact| fact.name == "Inner")
        .unwrap();
    let outer = forward
        .facts()
        .iter()
        .find(|fact| fact.name == "Outer")
        .unwrap();
    assert_eq!(inner.parent.as_ref(), Some(&outer.origin));
    let generated: Vec<_> = forward
        .facts()
        .iter()
        .filter(|fact| fact.name == "Generated")
        .collect();
    assert_eq!(generated.len(), 2);
    assert_ne!(generated[0].origin, generated[1].origin);
    assert_ne!(generated[0].expansion, generated[1].expansion);
    assert!(generated.iter().any(|fact| fact.parent.is_some()));

    let gamma = Input::new("gamma.hpp", "typedef int Status;");
    let expanded = extract([beta, gamma, alpha], &["-x", "c++"]).unwrap();
    let expanded_origins: Vec<_> = expanded
        .facts()
        .iter()
        .filter(|fact| fact.origin.tu != "gamma.hpp")
        .cloned()
        .collect();
    assert_eq!(expanded_origins, forward.facts());

    let error = extract(
        [Input::new("broken.hpp", "#include \"missing.hpp\"")],
        &["-x", "c++"],
    )
    .unwrap_err();
    assert!(error.to_string().contains("missing.hpp"));
}
