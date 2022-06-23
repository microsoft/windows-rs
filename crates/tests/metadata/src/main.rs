fn main() {
    use metadata::reader::*;

    println!("-- Windows --");

    let files = vec![File::new(r#"d:\git\windows-rs\crates\libs\metadata\default\Windows.winmd"#).unwrap()];
    let reader = &Reader::new(&files);
    let def = reader.get(TypeName::new("Windows.Foundation", "AsyncStatus")).next().unwrap();

    for field in reader.type_def_fields(def) {
        println!("{}", reader.field_name(field));
        let blob = reader.row_blob(field.0, 2);
        dbg!(blob.slice);
    }

    println!("-- TestWindows --");

    let files = vec![File::new(r#"C:\Users\kekerr\AppData\Local\Temp\test_metadata.winmd"#).unwrap()];
    let reader = &Reader::new(&files);
    let def = reader.get(TypeName::new("TestWindows.Foundation", "AsyncStatus")).next().unwrap();

    for field in reader.type_def_fields(def) {
        println!("{}", reader.field_name(field));
        let blob = reader.row_blob(field.0, 2);
        dbg!(blob.slice);
    }
}
