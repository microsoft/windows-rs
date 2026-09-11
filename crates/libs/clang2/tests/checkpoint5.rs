use windows_clang2::{FactData, Input, extract};

#[test]
fn records_and_pointer_typedefs_preserve_layout_and_dependencies() {
    windows_clang::ensure_libclang();

    let source = "\
typedef struct _POINT {
    int x;
    int y;
} POINT;
typedef POINT *PPOINT;
typedef const POINT *PCPOINT;

typedef union _NUMBER {
    int i;
    unsigned int u;
} NUMBER;

typedef struct _BOX {
    POINT origin;
    PPOINT mutable_point;
    PCPOINT const_point;
    NUMBER number;
} BOX;

typedef struct SAME {
    int value;
} SAME;

typedef struct _POINT_ARRAY {
    POINT values[3];
} POINT_ARRAY;

#pragma pack(push, 1)
typedef struct _PACKED {
    char byte;
    int value;
} PACKED;
#pragma pack(pop)

typedef struct alignas(16) _ALIGNED {
    int value;
} ALIGNED;

#pragma pack(push, 1)
typedef struct alignas(8) _PACKED_ALIGNED {
    char byte;
    int value;
} PACKED_ALIGNED;
#pragma pack(pop)

typedef struct _FLOATS {
    float single;
    double pair;
    wchar_t wide;
} FLOATS;
typedef const char **MIXED_POINTER;

struct OPAQUE;
typedef struct OPAQUE *POPAQUE;
";

    let snapshot = extract([Input::new("records.hpp", source)], &["-x", "c++"]).unwrap();
    let point = snapshot
        .facts()
        .iter()
        .find(|fact| fact.name == "_POINT" && fact.definition)
        .unwrap();
    let FactData::Record {
        fields,
        size,
        align,
        union,
        ..
    } = &point.data
    else {
        panic!("POINT record payload was not extracted");
    };
    assert!(!union);
    assert_eq!((*size, *align), (8, 4));
    assert_eq!(fields[0].name, "x");
    assert_eq!(fields[0].offset, 0);
    assert_eq!(fields[1].name, "y");
    assert_eq!(fields[1].offset, 32);

    let rdl = snapshot.emit("Records").unwrap();
    assert!(rdl.contains("struct POINT"));
    assert!(rdl.contains("x: i32"));
    assert!(rdl.contains("y: i32"));
    assert!(rdl.contains("type PPOINT = *mut POINT"));
    assert!(rdl.contains("type PCPOINT = *const POINT"));
    assert!(rdl.contains("union NUMBER"));
    assert!(rdl.contains("struct BOX"));
    assert!(rdl.contains("origin: POINT"));
    assert!(rdl.contains("mutable_point: PPOINT"));
    assert!(rdl.contains("const_point: PCPOINT"));
    assert!(rdl.contains("number: NUMBER"));
    assert_eq!(rdl.matches("struct SAME").count(), 1);
    assert!(rdl.contains("value: i32"));
    assert!(rdl.contains("struct POINT_ARRAY"));
    assert!(rdl.contains("values: [POINT; 3]"));
    assert!(rdl.contains("#[packed(1)]\n    struct PACKED"));
    assert!(rdl.contains("#[align(16)]\n    struct ALIGNED"));
    assert!(rdl.contains("#[packed(1)]\n    #[align(8)]\n    struct PACKED_ALIGNED"));
    assert!(rdl.contains("single: f32"));
    assert!(rdl.contains("pair: f64"));
    assert!(rdl.contains("wide: u16"));
    assert!(rdl.contains("type MIXED_POINTER = *const *const i8"));
    assert!(rdl.contains("struct OPAQUE {\n"));
    assert!(rdl.contains("type POPAQUE = *mut OPAQUE"));
    assert!(!rdl.contains("struct _POINT"));
    assert!(!rdl.contains("union _NUMBER"));

    let output = std::env::temp_dir().join(format!(
        "windows-clang2-records-{}.winmd",
        std::process::id()
    ));
    windows_rdl::reader()
        .input_text(&rdl)
        .output(&output)
        .write()
        .unwrap();
    std::fs::remove_file(output).unwrap();

    let unsupported = extract(
        [Input::new(
            "unsupported-layout.hpp",
            "typedef struct BAD_LAYOUT {\n\
                 char byte;\n\
                 int value __attribute__((aligned(8)));\n\
             } BAD_LAYOUT;\n\
             typedef BAD_LAYOUT *PBAD_LAYOUT;\n",
        )],
        &["-x", "c++"],
    )
    .unwrap()
    .emit("UnsupportedLayout")
    .unwrap_err();
    assert!(
        unsupported
            .to_string()
            .contains("record fields cannot reproduce Clang's layout")
    );

    let scratch = std::env::temp_dir().join(format!(
        "windows-clang2-record-alias-{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&scratch).unwrap();
    std::fs::write(
        scratch.join("shared.hpp"),
        "typedef struct _SHARED_RECORD { int value; } SHARED_RECORD;\n",
    )
    .unwrap();
    let include = format!("-I{}", scratch.display());
    let stable_name = extract(
        [Input::new(
            scratch.join("main.hpp").to_string_lossy(),
            "#include \"shared.hpp\"\ntypedef struct _SHARED_RECORD DIRECT_RECORD;\n",
        )],
        &["-x", "c++", &include],
    )
    .unwrap()
    .emit("StableName")
    .unwrap();
    assert!(stable_name.contains("struct SHARED_RECORD"));
    assert!(stable_name.contains("type DIRECT_RECORD = SHARED_RECORD"));
    assert!(!stable_name.contains("struct _SHARED_RECORD"));
    std::fs::remove_dir_all(scratch).unwrap();
}
