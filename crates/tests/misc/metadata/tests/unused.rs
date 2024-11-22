// use windows_metadata::*;

// #[test]
// fn test() {
//     let files = helpers::default_metadata();
//     let reader = Reader::filter(
//         files,
//         &["Windows", "BadNamespace", "Windows.AI"],
//         &["Windows.Foundation.Rect", "Windows.Foundation.BadType"],
//     );
//     let unused: Vec<&str> = reader.unused().collect();

//     assert_eq!(unused, ["Windows.Foundation.BadType", "BadNamespace"]);
// }
