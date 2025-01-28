use windows_clang::*;

#[test]
fn parse() {
    let index = Index::new();
    let tu = index.tu(c"tests/test.cpp").unwrap();
    let cursor = tu.cursor();

    let mut names = vec![];

    cursor.visit(|child| {
        names.push(child.name().to_string());
        VisitResult::Continue
    });

    assert_eq!(names, ["TypeA", "TypeB",]);

    let mut names = vec![];
    let mut kinds = vec![];
    let mut types = vec![];

    cursor.visit(|child| {
        names.push(child.name().to_string());
        kinds.push(child.kind());
        types.push(child.ty());
        VisitResult::Recurse
    });

    assert_eq!(names, ["TypeA", "a", "b", "TypeB", "c", "d",]);

    assert_eq!(
        kinds,
        [
            CursorKind::StructDecl,
            CursorKind::FieldDecl,
            CursorKind::FieldDecl,
            CursorKind::StructDecl,
            CursorKind::FieldDecl,
            CursorKind::FieldDecl
        ]
    );
}

#[test]
fn find() {
    let index = Index::new();
    let tu = index.tu(c"tests/test.cpp").unwrap();
    let cursor = tu.cursor();

    let cursor = cursor.find(|next| next.name() == "TypeA").unwrap();
    assert_eq!(cursor.name(), "TypeA");
    assert_eq!(cursor.kind(), CursorKind::StructDecl);
    assert_eq!(cursor.ty().kind(), TypeKind::Record);
}

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
