use windows_clang::*;

#[test]
fn fields() {
    let index = CXIndex::new();
    let tu = index.tu(c"tests/fields.cpp", 0).unwrap();
    let cursor = tu.cursor();

    let mut child = None;

    cursor.visit(|next| {
        child = Some(next);
        CXChildVisit_Break
    });

    let child = child.unwrap();
    assert_eq!(child.name().as_str(), "Type");
    assert_eq!(child.kind(), CXCursor_StructDecl);

    let ty = child.ty();
    assert_eq!(ty.kind(), CXType_Record);

    let mut fields = vec![];

    ty.visit(|next| {
        fields.push(next);
        CXVisit_Continue
    });

    assert_eq!(fields.len(), 3);

    assert_eq!(fields[0].name().as_str(), "a");
    assert_eq!(fields[1].name().as_str(), "b");
    assert_eq!(fields[2].name().as_str(), "c");

    assert_eq!(fields[0].ty().kind(), CXType_Int);
    assert_eq!(fields[1].ty().kind(), CXType_Pointer);
    assert_eq!(fields[2].ty().kind(), CXType_Bool);
}
