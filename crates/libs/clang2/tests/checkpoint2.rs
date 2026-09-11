use windows_clang2::{Input, TypeRef, extract};

#[test]
fn constants_use_final_preprocessed_value_and_type() {
    windows_clang::ensure_libclang();

    let source = "\
typedef unsigned short Status;
typedef Status Chained;
typedef unsigned short Shadow;
typedef unsigned short FunctionShadow;
enum State : unsigned int { Ready = 1 };

#define COMPUTED ((Status)(1 + 2))
#define CHAINED ((Chained)(2 + 2))
#define ENUM_VALUE ((State)1)
#define REDEFINED ((Status)4)
#undef REDEFINED
#define REDEFINED 9
#define TERMINAL ((Status)5)
#undef TERMINAL
#define SHADOWED ((Shadow)(1 + 2))
#define Shadow unsigned int
#define FunctionShadow(value) value
#define FUNCTION_SHADOW ((FunctionShadow)(1 + 2))
#define PAIR 1, 2
#define A_POISON (
#define Z_AFTER_POISON 12
#define NEGATIVE -5
#define HIGH_U32 0x80000000u
#define MAX_U64 0xffffffffffffffffull
extern int runtime_value;
#define RUNTIME_VALUE ((Status)runtime_value)
";

    let snapshot = extract([Input::new("constants.hpp", source)], &["-x", "c++"]).unwrap();
    let rdl = snapshot.emit("Clang2").unwrap();

    assert!(rdl.contains("type Status = u16"));
    assert!(rdl.contains("type Chained = Status"));
    assert!(rdl.contains("enum State"));
    assert!(rdl.contains("Ready = 1"));
    assert!(
        rdl.contains("const COMPUTED: Status = 3"),
        "{}\n{rdl}",
        snapshot.dump()
    );
    assert!(rdl.contains("const CHAINED: Chained = 4"));
    assert!(rdl.contains("const ENUM_VALUE: State = 1"));
    assert!(rdl.contains("const REDEFINED: i32 = 9"));
    assert!(rdl.contains("const SHADOWED: u32 = 3"));
    assert!(rdl.contains("const FUNCTION_SHADOW: FunctionShadow = 3"));
    assert!(rdl.contains("const Z_AFTER_POISON: i32 = 12"));
    assert!(rdl.contains("const NEGATIVE: i32 = -5"));
    assert!(rdl.contains("const HIGH_U32: u32 = 2147483648"));
    assert!(rdl.contains("const MAX_U64: u64 = 18446744073709551615"));
    assert!(!rdl.contains("const PAIR"));
    assert!(!rdl.contains("const A_POISON"));
    assert!(!rdl.contains("const TERMINAL"));
    assert!(!rdl.contains("const RUNTIME_VALUE"));

    let status = snapshot
        .facts()
        .iter()
        .find(|fact| fact.name == "Status")
        .unwrap();
    let computed = snapshot
        .constants()
        .iter()
        .find(|constant| constant.name == "COMPUTED")
        .unwrap();
    let TypeRef::Named { declaration, .. } = &computed.ty else {
        panic!("COMPUTED lost its named type");
    };
    assert_eq!(declaration, &status.spelling);

    let redefined = snapshot
        .constants()
        .iter()
        .find(|constant| constant.name == "REDEFINED")
        .unwrap();
    assert_ne!(redefined.root, redefined.definition);

    let output = std::env::temp_dir().join(format!("windows-clang2-{}.winmd", std::process::id()));
    windows_rdl::reader()
        .input_text(&rdl)
        .output(&output)
        .write()
        .unwrap();
    std::fs::remove_file(output).unwrap();

    let scratch = std::env::temp_dir().join(format!("windows-clang2-{}", std::process::id()));
    std::fs::create_dir_all(&scratch).unwrap();
    std::fs::write(
        scratch.join("redefine.hpp"),
        "#undef INCLUDED_REDEFINED\n#define INCLUDED_REDEFINED 11\n",
    )
    .unwrap();
    let main = scratch.join("main.hpp");
    let source = "\
typedef unsigned short IncludedStatus;
#define INCLUDED_REDEFINED ((IncludedStatus)4)
#include \"redefine.hpp\"
";
    let include_arg = format!("-I{}", scratch.display());
    let snapshot = extract(
        [Input::new(main.to_string_lossy(), source)],
        &["-x", "c++", &include_arg],
    )
    .unwrap();
    let rdl = snapshot.emit("IncludedRedefinition").unwrap();
    assert!(rdl.contains("const INCLUDED_REDEFINED: i32 = 11"), "{rdl}");
    assert!(!rdl.contains("const INCLUDED_REDEFINED: IncludedStatus"));
    let constant = snapshot
        .constants()
        .iter()
        .find(|constant| constant.name == "INCLUDED_REDEFINED")
        .unwrap();
    let definition = snapshot
        .facts()
        .iter()
        .find(|fact| fact.origin == constant.definition)
        .unwrap();
    assert!(!definition.main_file);
    assert!(definition.spelling.file.ends_with("redefine.hpp"));
    std::fs::remove_dir_all(scratch).unwrap();
}
