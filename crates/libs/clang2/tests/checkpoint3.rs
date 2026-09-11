use windows_clang2::{Input, extract};

#[test]
fn planning_is_order_independent_and_dependencies_stay_tu_local() {
    windows_clang::ensure_libclang();

    let scratch = std::env::temp_dir().join(format!("windows-clang2-plan-{}", std::process::id()));
    std::fs::create_dir_all(&scratch).unwrap();
    std::fs::write(
        scratch.join("dependency.hpp"),
        "typedef unsigned short Included;\n\
         typedef Included Chained;\n\
         typedef unsigned short StaleAlias;\n\
         typedef enum SharedState : unsigned int { SharedReady = 1 } SharedState;\n",
    )
    .unwrap();

    let value_source = "\
#include \"dependency.hpp\"
#define INCLUDED_VALUE ((Chained)7)
#define SHARED_STATE_A ((SharedState)1)
#define Collision ((StaleAlias)5)
";
    let type_source = "\
#include \"dependency.hpp\"
typedef unsigned short Collision;
#define COLLISION_VALUE ((Collision)3)
#define SECOND_INCLUDED ((Chained)8)
#define SHARED_STATE_B ((SharedState)1)
";
    let value = Input::new(scratch.join("value.hpp").to_string_lossy(), value_source);
    let ty = Input::new(scratch.join("type.hpp").to_string_lossy(), type_source);
    let include = format!("-I{}", scratch.display());
    let args = ["-x", "c++", include.as_str()];

    let forward = extract([value.clone(), ty.clone()], &args)
        .unwrap()
        .emit("Plan")
        .unwrap();
    let reverse = extract([ty, value], &args).unwrap().emit("Plan").unwrap();

    assert_eq!(forward, reverse);
    assert!(forward.contains("type Included = u16"));
    assert!(forward.contains("type Chained = Included"));
    assert!(forward.contains("const INCLUDED_VALUE: Chained = 7"));
    assert!(forward.contains("type Collision = u16"));
    assert!(forward.contains("const COLLISION_VALUE: Collision = 3"));
    assert!(forward.contains("const SECOND_INCLUDED: Chained = 8"));
    assert_eq!(forward.matches("enum SharedState").count(), 1);
    assert!(forward.contains("const SHARED_STATE_A: SharedState = 1"));
    assert!(forward.contains("const SHARED_STATE_B: SharedState = 1"));
    assert!(!forward.contains("const Collision"));
    assert!(!forward.contains("StaleAlias"));

    let output = scratch.join("out.winmd");
    windows_rdl::reader()
        .input_text(&forward)
        .output(&output)
        .write()
        .unwrap();

    let ambiguous = extract(
        [
            Input::new("left.hpp", "typedef unsigned short Status;"),
            Input::new("right.hpp", "typedef unsigned int Status;"),
        ],
        &["-x", "c++"],
    )
    .unwrap()
    .emit("Ambiguous")
    .unwrap_err();
    assert!(
        ambiguous
            .to_string()
            .contains("ambiguous type root `Status`")
    );

    let duplicate_constant = extract(
        [
            Input::new("constant-a.hpp", "#define SAME_VALUE 7"),
            Input::new("constant-b.hpp", "#define SAME_VALUE 7"),
        ],
        &["-x", "c++"],
    )
    .unwrap()
    .emit("DuplicateConstant")
    .unwrap();
    assert_eq!(duplicate_constant.matches("const SAME_VALUE").count(), 1);

    let dependency_collision = extract(
        [
            Input::new(
                scratch.join("dependency-value.hpp").to_string_lossy(),
                "#include \"dependency.hpp\"\n#define STALE_USE ((StaleAlias)5)",
            ),
            Input::new("stale-value.hpp", "#define StaleAlias 5"),
        ],
        &args,
    )
    .unwrap()
    .emit("DependencyCollision")
    .unwrap();
    assert!(dependency_collision.contains("type StaleAlias = u16"));
    assert!(dependency_collision.contains("const STALE_USE: StaleAlias = 5"));
    assert!(!dependency_collision.contains("const StaleAlias"));

    std::fs::write(
        scratch.join("first-conflict.hpp"),
        "typedef unsigned short Widget;\ntypedef unsigned short Conflict;\n",
    )
    .unwrap();
    std::fs::write(
        scratch.join("second-conflict.hpp"),
        "typedef unsigned int Conflict;\n",
    )
    .unwrap();
    let discarded_dependencies = extract(
        [
            Input::new(
                scratch.join("uses-first.hpp").to_string_lossy(),
                "#include \"first-conflict.hpp\"\n\
                 #define USES_WIDGET ((Widget)1)\n\
                 #define USES_CONFLICT ((Conflict)1)\n",
            ),
            Input::new(
                scratch.join("discarded-value.hpp").to_string_lossy(),
                "#include \"second-conflict.hpp\"\n#define Widget ((Conflict)1)\n",
            ),
        ],
        &args,
    )
    .unwrap()
    .emit("DiscardedDependencies")
    .unwrap();
    assert!(discarded_dependencies.contains("type Widget = u16"));
    assert!(discarded_dependencies.contains("type Conflict = u16"));
    assert!(!discarded_dependencies.contains("type Conflict = u32"));
    assert!(!discarded_dependencies.contains("const Widget"));

    let incomplete = extract(
        [Input::new(
            "incomplete.hpp",
            "enum Missing : int;\ntypedef Missing Alias;",
        )],
        &["-x", "c++"],
    )
    .unwrap()
    .emit("Incomplete")
    .unwrap_err();
    assert!(
        incomplete
            .to_string()
            .contains("type root `Missing` is not emittable")
    );

    let public_enum_snapshot = extract(
        [Input::new(
            "enum.hpp",
            "typedef enum State : unsigned int { Ready = 1 } State;\n\
             #define CURRENT_STATE ((State)1)\n",
        )],
        &["-x", "c++"],
    )
    .unwrap();
    let public_enum = public_enum_snapshot.emit("PublicEnum").unwrap();
    assert_eq!(
        public_enum.matches("enum State").count(),
        1,
        "{}\n{public_enum}",
        public_enum_snapshot.dump()
    );
    assert!(public_enum.contains("const CURRENT_STATE: State = 1"));

    let renamed_enum = extract(
        [Input::new(
            "renamed-enum.hpp",
            "typedef enum _RENAMED : int { RenamedValue = 1 } RENAMED;\n\
             typedef RENAMED RENAMED_CHAIN;\n\
             #define CURRENT_RENAMED ((RENAMED)1)\n",
        )],
        &["-x", "c++"],
    )
    .unwrap()
    .emit("RenamedEnum")
    .unwrap();
    assert!(renamed_enum.contains("enum RENAMED"));
    assert!(renamed_enum.contains("type RENAMED_CHAIN = RENAMED"));
    assert!(renamed_enum.contains("const CURRENT_RENAMED: RENAMED = 1"));
    assert!(!renamed_enum.contains("enum _RENAMED"));
    assert!(!renamed_enum.contains("= _RENAMED"));
    assert!(!renamed_enum.contains(": _RENAMED"));

    let forward_enum = extract(
        [Input::new(
            "forward-enum.hpp",
            "enum Forward : int;\n\
             enum Forward : int { ForwardValue = 1 };\n\
             #define CURRENT_FORWARD ((Forward)1)\n",
        )],
        &["-x", "c++"],
    )
    .unwrap()
    .emit("ForwardEnum")
    .unwrap();
    assert_eq!(forward_enum.matches("enum Forward").count(), 1);
    assert!(forward_enum.contains("const CURRENT_FORWARD: Forward = 1"));

    std::fs::remove_dir_all(scratch).unwrap();
}
