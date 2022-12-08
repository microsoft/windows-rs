use windows::Storage::Search::*;

#[test]
fn test() {
    // SortyEntry is a WinRT struct with non-blittable fields. This just validates that it can
    // be used directly and isn't wrapped inside ManuallyDrop.

    let mut se = SortEntry::default();
    assert!(se.PropertyName.is_empty());
    se.PropertyName = "name".into();
    let copy = se.clone();
    assert_eq!(copy.PropertyName, "name");
}
