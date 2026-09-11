use windows_clang2::{FactData, Input, extract};
use windows_metadata::HasAttributes;

const METADATA_RDL: &str = include_str!("../../../../metadata/metadata.rdl");

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

typedef struct _FLAGS {
    unsigned int low : 3;
    unsigned int : 2;
    unsigned int high : 3;
    unsigned int wide : 24;
    unsigned int next : 16;
} FLAGS;

typedef struct _NESTED {
    struct {
        POINT point;
        int value;
    } named;
    union {
        int integer;
        float floating;
    };
} NESTED;

typedef struct {
    int item;
} ANONYMOUS_TYPE;

typedef union {
    unsigned int bits;
    float value;
} ANONYMOUS_UNION;

struct OPAQUE;
typedef struct OPAQUE *POPAQUE;
typedef const char *PCSTR;
typedef char *PSTR;
typedef const wchar_t *PCWSTR;
typedef wchar_t *PWSTR;

extern \"C\" int TranslatePoint(POINT point, const POINT *input, POINT *output);
extern \"C\" void ResetBox(BOX *box);
typedef int (*COMPARE_POINT)(const POINT *left, const POINT *right);
extern \"C\" int SortPoints(POINT *points, unsigned int count, COMPARE_POINT compare);
typedef int type;
extern \"C\" type KeywordType(type value);
#define IN __attribute__((annotate(\"_In_\")))
#define OUT_OPT __attribute__((annotate(\"_Out_opt_\")))
#define INOUT __attribute__((annotate(\"_Inout_\")))
#define RESERVED __attribute__((annotate(\"_Reserved_\")))
#define READS(c) __attribute__((annotate(\"_In_reads_(\" #c \")\")))
#define WRITES_BYTES(c) __attribute__((annotate(\"_Out_writes_bytes_(\" #c \")\")))
#define WRITES_TO(s, c) __attribute__((annotate(\"_Out_writes_to_(\" #s \",\" #c \")\")))
#define IN_Z __attribute__((annotate(\"_In_z_\")))
#define OUT_Z __attribute__((annotate(\"_Out_z_\")))
#define READS_Z(c) __attribute__((annotate(\"_In_reads_z_(\" #c \")\")))
#define COM_OUT __attribute__((annotate(\"_COM_Outptr_\")))
extern \"C\" int Annotated(IN const POINT *input, OUT_OPT POINT *output,
                          INOUT unsigned int *flags, RESERVED void *context);
extern \"C\" int Buffers(READS(count) const POINT *points, unsigned int count,
                        WRITES_BYTES(bytes) void *buffer, unsigned int bytes,
                        READS(0x10) const char *fixed,
                        WRITES_TO(capacity, written) char *text,
                        unsigned int capacity, unsigned int *written);
extern \"C\" int Strings(IN_Z const char *narrow, OUT_Z wchar_t *wide,
                        READS_Z(count) const char *counted, unsigned int count);
extern \"C\" int Create(COM_OUT void **object);

struct IBase {
    virtual int First(int value) = 0;
    virtual int Over(int integer) = 0;
    virtual int Over(float floating) = 0;
};
struct IDerived : IBase {
    virtual int First(int value) override = 0;
    virtual int Next(IBase *other, IBase **out_other, COM_OUT void **object) = 0;
};
typedef struct _IAlias {
    virtual int AliasMethod() = 0;
} IAlias;
extern \"C\" int UseAlias(IAlias *value);
extern \"C\" IBase *GetBase();
typedef IBase *(*GET_BASE)();
typedef IBase *PIBASE;
struct INTERFACE_HOLDER {
    IBase *value;
};
struct __declspec(uuid(\"12345678-1234-abcd-9876-0123456789ab\")) IGuid {
    virtual void Guided() = 0;
};
";

    let snapshot = extract(
        [Input::new("records.hpp", source)],
        &["-x", "c++", "-fms-extensions"],
    )
    .unwrap();
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

    let rdl = snapshot.emit_with_library("Records", "test.dll").unwrap();
    assert!(
        snapshot
            .emit("Records")
            .unwrap_err()
            .to_string()
            .contains("requires an import library")
    );
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
    assert!(rdl.contains("_bitfield1: u32"));
    assert!(rdl.contains("low: 3"));
    assert!(rdl.contains("_: 2"));
    assert!(rdl.contains("high: 3"));
    assert!(rdl.contains("_bitfield2: u32"));
    assert!(rdl.contains("next: 16"));
    assert!(rdl.contains("struct NESTED"));
    assert!(rdl.contains("named: struct {"));
    assert!(rdl.contains("point: POINT"));
    assert!(rdl.contains("Anonymous: union {"));
    assert!(rdl.contains("integer: i32"));
    assert!(rdl.contains("floating: f32"));
    assert!(rdl.contains("struct ANONYMOUS_TYPE"));
    assert!(rdl.contains("item: i32"));
    assert!(rdl.contains("union ANONYMOUS_UNION"));
    assert!(rdl.contains("struct OPAQUE {\n"));
    assert!(rdl.contains("type POPAQUE = *mut OPAQUE"));
    assert!(rdl.contains(
        "extern \"C\" fn TranslatePoint(point: POINT, input: *const POINT, output: *mut POINT) -> i32"
    ));
    assert!(rdl.contains("extern \"C\" fn ResetBox(r#box: *mut BOX);"));
    assert!(rdl.contains(
        "extern \"C\" fn COMPARE_POINT(param0: *const POINT, param1: *const POINT) -> i32"
    ));
    assert!(rdl.contains(
        "extern \"C\" fn SortPoints(points: *mut POINT, count: u32, compare: COMPARE_POINT) -> i32"
    ));
    assert!(rdl.contains("type r#type = i32"));
    assert!(rdl.contains("extern \"C\" fn KeywordType(value: r#type) -> r#type"));
    assert!(rdl.contains("#[in] input: *const POINT"));
    assert!(rdl.contains("#[out] #[opt] output: *mut POINT"));
    assert!(rdl.contains("#[in] #[out] flags: *mut u32"));
    assert!(rdl.contains("#[reserved] context: *mut void"));
    assert!(rdl.contains("#[len_param(1)] #[in] points: *const POINT"));
    assert!(rdl.contains("#[size_param(3)] #[out] buffer: *mut void"));
    assert!(rdl.contains("#[len_const(16)] #[in] fixed: *const i8"));
    assert!(rdl.contains("#[len_param(6)] #[out] text: *mut i8"));
    assert!(rdl.contains(
        "extern \"C\" fn Strings(#[in] narrow: PCSTR, #[out] wide: PWSTR, \
         #[len_param(3)] #[in] counted: *const i8, count: u32)"
    ));
    assert!(rdl.contains("extern \"C\" fn Create(#[iid_is] #[out] object: *mut *mut void)"));
    assert!(rdl.contains("#[no_guid]\n    interface IBase"));
    assert!(rdl.contains("fn First(&self, value: i32) -> i32"));
    let float_over = rdl.find("fn Over(&self, floating: f32)").unwrap();
    let integer_over = rdl.find("fn Over(&self, integer: i32)").unwrap();
    assert!(float_over < integer_over);
    assert!(rdl.contains("#[no_guid]\n    interface IDerived: IBase"));
    assert!(!rdl.contains("interface IDerived: IBase {\n        fn First"));
    assert!(rdl.contains(
        "fn Next(&self, other: IBase, out_other: *mut IBase, \
             #[iid_is] #[out] object: *mut *mut void) -> i32"
    ));
    assert!(rdl.contains("interface IAlias"));
    assert!(!rdl.contains("interface _IAlias"));
    assert!(rdl.contains("extern \"C\" fn UseAlias(value: IAlias) -> i32"));
    assert!(rdl.contains("extern \"C\" fn GetBase() -> IBase"));
    assert!(rdl.contains("extern \"C\" fn GET_BASE() -> IBase"));
    assert!(rdl.contains("type PIBASE = IBase"));
    assert!(rdl.contains("struct INTERFACE_HOLDER {\n        value: IBase,"));
    assert!(rdl.contains("#[guid(0x12345678_1234_abcd_9876_0123456789ab)]\n    interface IGuid"));
    assert!(!rdl.contains("struct _POINT"));
    assert!(!rdl.contains("union _NUMBER"));

    let output = std::env::temp_dir().join(format!(
        "windows-clang2-records-{}.winmd",
        std::process::id()
    ));
    windows_rdl::reader()
        .input_text(METADATA_RDL)
        .input_text(&rdl)
        .output(&output)
        .write()
        .unwrap_or_else(|error| panic!("{error}\n{rdl}"));
    let index = windows_metadata::reader::Index::read(&output).unwrap();
    let windows_metadata::reader::Item::Fn(function) =
        index.expect_item("Records", "TranslatePoint")
    else {
        panic!("TranslatePoint was not emitted as an imported function");
    };
    let import = function.impl_map().unwrap();
    assert_eq!(import.import_name(), "TranslatePoint");
    assert_eq!(import.import_scope().name(), "test.dll");
    let windows_metadata::reader::Item::Type(callback) =
        index.expect_item("Records", "COMPARE_POINT")
    else {
        panic!("COMPARE_POINT was not emitted as a callback type");
    };
    assert_eq!(callback.name(), "COMPARE_POINT");
    let windows_metadata::reader::Item::Fn(annotated) = index.expect_item("Records", "Annotated")
    else {
        panic!("Annotated was not emitted as an imported function");
    };
    let params = annotated.params_by_sequence(4).unwrap();
    let [Some(input), Some(output_param), Some(flags), Some(context)] = params.params() else {
        panic!("Annotated parameter metadata is incomplete");
    };
    assert_eq!(
        input.direction(),
        windows_metadata::reader::ParamDirection::Input
    );
    assert_eq!(
        output_param.direction(),
        windows_metadata::reader::ParamDirection::Output
    );
    assert!(output_param.is_optional());
    assert_eq!(
        flags.direction(),
        windows_metadata::reader::ParamDirection::InputOutput
    );
    assert!(context.is_reserved());
    let windows_metadata::reader::Item::Fn(buffers) = index.expect_item("Records", "Buffers")
    else {
        panic!("Buffers was not emitted as an imported function");
    };
    let params = buffers.params_by_sequence(8).unwrap();
    let [
        Some(points),
        _,
        Some(buffer),
        _,
        Some(fixed),
        Some(text),
        _,
        _,
    ] = params.params()
    else {
        panic!("Buffers parameter metadata is incomplete");
    };
    use windows_metadata::reader::BufferRelationship;
    assert_eq!(
        points.buffer_relationship(),
        Some(BufferRelationship::ElementsParam(1))
    );
    assert_eq!(
        buffer.buffer_relationship(),
        Some(BufferRelationship::BytesParam(3))
    );
    assert_eq!(
        fixed.buffer_relationship(),
        Some(BufferRelationship::ElementsConst(16))
    );
    assert_eq!(
        text.buffer_relationship(),
        Some(BufferRelationship::ElementsParam(6))
    );
    let windows_metadata::reader::Item::Fn(create) = index.expect_item("Records", "Create") else {
        panic!("Create was not emitted as an imported function");
    };
    let params = create.params_by_sequence(1).unwrap();
    let [Some(object)] = params.params() else {
        panic!("Create parameter metadata is incomplete");
    };
    assert!(
        object
            .attributes()
            .any(|attribute| attribute.name() == "ComOutPtrAttribute")
    );
    std::fs::remove_file(output).unwrap();
}

#[test]
fn unsupported_sal_size_relations_are_reported() {
    windows_clang::ensure_libclang();

    for (annotation, expected) in [
        ("_In_reads_(count + 1)", "unsupported SAL size expression"),
        (
            "_In_reads_(missing)",
            "unresolved SAL size parameter `missing`",
        ),
        (
            "_Out_writes_bytes_(4)",
            "constant byte-size SAL annotations are unsupported",
        ),
    ] {
        let source = format!(
            "extern \"C\" void Invalid(\
             __attribute__((annotate(\"{annotation}\"))) void *buffer, unsigned int count);"
        );
        let snapshot = extract([Input::new("invalid.hpp", source)], &["-x", "c++"]).unwrap();
        assert!(
            snapshot
                .emit_with_library("Invalid", "test.dll")
                .unwrap_err()
                .to_string()
                .contains(expected)
        );
    }

    let collision = extract(
        [Input::new(
            "function-collision.hpp",
            "struct conflict { int value; };\nextern \"C\" int conflict(int value);\n",
        )],
        &["-x", "c++"],
    )
    .unwrap()
    .emit_with_library("FunctionCollision", "test.dll")
    .unwrap_err();
    assert!(
        collision
            .to_string()
            .contains("type and function roots collide on `conflict`")
    );

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

    let zero_width = extract(
        [Input::new(
            "zero-width.hpp",
            "typedef struct ZERO_WIDTH {\n\
                 unsigned int first : 1;\n\
                 unsigned int : 0;\n\
                 unsigned int second : 1;\n\
             } ZERO_WIDTH;\n\
             typedef ZERO_WIDTH *PZERO_WIDTH;\n",
        )],
        &["-x", "c++"],
    )
    .unwrap()
    .emit("ZeroWidth")
    .unwrap();
    assert!(zero_width.contains("_bitfield1: u32"));
    assert!(zero_width.contains("_bitfield2: u32"));
    assert!(zero_width.contains("first: 1"));
    assert!(zero_width.contains("second: 1"));
    let zero_output = std::env::temp_dir().join(format!(
        "windows-clang2-zero-width-{}.winmd",
        std::process::id()
    ));
    windows_rdl::reader()
        .input_text(&zero_width)
        .output(&zero_output)
        .write()
        .unwrap();
    std::fs::remove_file(zero_output).unwrap();

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

#[test]
fn invalid_interface_shapes_are_unsupported_facts() {
    windows_clang::ensure_libclang();
    let source = "\
struct DATA {};
struct IBadBase : DATA {
    virtual void Method() = 0;
};
struct IBadDestructor {
    virtual ~IBadDestructor() = 0;
    virtual void Method() = 0;
};
";
    let snapshot = extract([Input::new("invalid.hpp", source)], &["-x", "c++"]).unwrap();
    for (name, expected) in [
        ("IBadBase", "interface base is not an interface"),
        (
            "IBadDestructor",
            "interface has a constructor or destructor",
        ),
    ] {
        let fact = snapshot
            .facts()
            .iter()
            .find(|fact| fact.name == name)
            .unwrap();
        let FactData::Unsupported { reason } = &fact.data else {
            panic!("{name} was not classified as unsupported");
        };
        assert!(reason.contains(expected), "{reason}");
    }
}
