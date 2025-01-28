use windows_clang::*;

#[test]
fn find() {
    let index = Index::new();
    let tu = index.tu(c"tests/fields.cpp").unwrap();
    let cursor = tu.cursor();

    let ty = cursor.find(|next| next.name() == "Type").unwrap();
    assert_eq!(ty.name(), "Type");
    assert_eq!(ty.kind(), CursorKind::StructDecl);
    assert_eq!(ty.ty().kind(), TypeKind::Record);

    let field = ty.ty().find(|next| next.name() == "c").unwrap();

    assert_eq!(field.name(), "c");
    assert_eq!(field.kind(), CursorKind::FieldDecl);
    assert_eq!(field.ty().kind(), TypeKind::Bool);

    assert!(cursor.find(|next| next.name() == "TypeNotFound").is_none());
    assert!(ty.ty().find(|next| next.name() == "FieldNotFound").is_none());
}
