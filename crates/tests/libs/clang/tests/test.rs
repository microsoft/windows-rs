use windows_clang::*;

#[test]
fn parse() {
    let index = CXIndex::new();
    let tu = index.tu(c"tests/test.cpp", 0).unwrap();
    let cursor = tu.cursor();

    let mut names = vec![];

    cursor.visit(|child| {
        names.push(child.name().to_string());
        CXChildVisit_Continue
    });

    assert_eq!(names, ["TypeA", "TypeB",]);

    let mut names = vec![];
    let mut kinds = vec![];
    let mut types = vec![];

    cursor.visit(|child| {
        names.push(child.name().to_string());
        kinds.push(child.kind());
        types.push(child.ty());
        CXChildVisit_Recurse
    });

    assert_eq!(names, ["TypeA", "a", "b", "TypeB", "c", "d",]);

    assert_eq!(
        kinds,
        [
            CXCursor_StructDecl,
            CXCursor_FieldDecl,
            CXCursor_FieldDecl,
            CXCursor_StructDecl,
            CXCursor_FieldDecl,
            CXCursor_FieldDecl
        ]
    );
}

#[test]
fn type_kind() {
    assert_eq!(format!("{CXType_Bool}"), "Bool");
    assert_eq!(format!("{CXType_Bool:?}"), "Bool");
}

#[test]
fn cursor_kind() {
    assert_eq!(format!("{CXCursor_StructDecl}"), "StructDecl");
    assert_eq!(format!("{CXCursor_StructDecl:?}"), "StructDecl");
}
