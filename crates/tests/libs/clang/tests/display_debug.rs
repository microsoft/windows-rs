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
