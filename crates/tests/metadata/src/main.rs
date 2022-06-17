fn main() {
    use metadata::reader::*;

    let files = vec![File::new(r#"d:\git\windows-rs\crates\libs\metadata\default\Windows.winmd"#).unwrap()];
    let reader = &Reader::new(&files);
    let def = reader.get(TypeName::new("Windows.Foundation", "AsyncStatus")).next().unwrap();
    let field = reader.type_def_fields(def).next().unwrap();
    let blob = reader.row_blob(field.0, 2);
    dbg!(blob.slice);

    let files = vec![File::new(r#"C:\Users\kekerr\AppData\Local\Temp\test_metadata.winmd"#).unwrap()];
    let reader = &Reader::new(&files);
    let def = reader.get(TypeName::new("TestWindows.Foundation", "AsyncStatus")).next().unwrap();
    let field = reader.type_def_fields(def).next().unwrap();
    let blob = reader.row_blob(field.0, 2);
    dbg!(blob.slice);
}
