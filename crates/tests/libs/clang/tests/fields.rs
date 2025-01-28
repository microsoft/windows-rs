use windows_clang::*;

#[test]
fn fields() {
    let index = Index::new();
    let tu = index.tu(c"tests/fields.cpp").unwrap();
    let cursor = tu.cursor();

    let mut child = None;

    cursor.visit(|next| {
        child = Some(next);
        VisitResult::Break
    });

    let child = child.unwrap();
    assert_eq!(child.name().as_str(), "Type");
    assert_eq!(child.kind(), CursorKind::StructDecl);

    let ty = child.ty();
    assert_eq!(ty.kind(), TypeKind::Record);

    let mut fields = vec![];

    ty.visit(|next| {
        fields.push(next);
        VisitResult::Continue
    });

    assert_eq!(fields.len(), 3);

    assert_eq!(fields[0].name().as_str(), "a");
    assert_eq!(fields[1].name().as_str(), "b");
    assert_eq!(fields[2].name().as_str(), "c");

    assert_eq!(fields[0].ty().kind(), TypeKind::Int);
    assert_eq!(fields[1].ty().kind(), TypeKind::Pointer);
    assert_eq!(fields[2].ty().kind(), TypeKind::Bool);
}

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
}
