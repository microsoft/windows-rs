#![cfg(all(target_arch = "x86_64", target_env = "msvc"))]

use windows_clang::*;

#[test]
fn recurse() {
    let index = Index::new();
    let tu = index.tu(c"tests/visit.cpp").unwrap();
    let cursor = tu.cursor();
    let mut recurse = vec![];

    cursor.visit(|child, parent| {
        recurse.push(format!("{} - {}", parent.name(), child.name()));
        Visit::Recurse
    });

    assert_eq!(
        recurse,
        [
            "tests/visit.cpp - TypeA",
            "TypeA - a",
            "TypeA - b",
            "tests/visit.cpp - TypeB",
            "TypeB - c",
            "TypeB - d"
        ]
    );
}

#[test]
fn children() {
    let index = Index::new();
    let tu = index.tu(c"tests/visit.cpp").unwrap();
    let cursor = tu.cursor();
    let mut children = vec![];

    cursor.visit(|child, parent| {
        children.push(format!("{} - {}", parent.name(), child.name()));
        Visit::Continue
    });

    assert_eq!(
        children,
        ["tests/visit.cpp - TypeA", "tests/visit.cpp - TypeB",]
    );
}

#[test]
fn fields() {
    let index = Index::new();
    let tu = index.tu(c"tests/visit.cpp").unwrap();
    let cursor = tu.cursor().find(|child| child.name() == "TypeB").unwrap();
    let mut fields = vec![];

    cursor.ty().visit(|field| {
        fields.push(field.name().to_string());
        Visit::Continue
    });

    assert_eq!(fields, ["c", "d",]);
}
