use windows_clang::*;

#[test]
fn type_kind() {
    assert_eq!(format!("{}", TypeKind::Bool), "Bool");
    assert_eq!(format!("{:?}", TypeKind::Bool), "Bool");
}

#[test]
fn cursor_kind() {
    assert_eq!(format!("{}", CursorKind::StructDecl), "StructDecl");
    assert_eq!(format!("{:?}", CursorKind::StructDecl), "StructDecl");
}

#[test]
fn tu() {
    let index = Index::new();
    let tu = index.tu(c"tests/display_debug.cpp").unwrap();

    assert_eq!(format!("{tu}"), "tests/display_debug.cpp");
    assert_eq!(format!("{tu:?}"), "tests/display_debug.cpp");
}
