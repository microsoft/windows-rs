use windows::Storage::Search::*;

#[test]
fn test() {
    // SortyEntry is a rare WinRT struct with non-blittable fields.

    let mut se = SortEntry::default();
    assert!(se.PropertyName.is_empty());
    se.PropertyName = "name".into();
    let copy = se.clone();
    assert_eq!(copy.PropertyName, "name");
}
