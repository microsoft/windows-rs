use windows_metadata::*;

fn main() {
    windows_metagen::test();
    let file = windows_metadata::File::new("/git/test.winmd");
    let row_count = file.type_def_table().row_count;
    for row in 0..row_count {
        let def: TypeDef = Row::new(row, TableIndex::TypeDef, unsafe { &*(&file as *const _) }).into();
        let type_name = def.type_name();
        println!("{}", type_name);
        for field in def.fields() {
            let name = field.name();
            println!("field: {}", name);
        }

        for method in def.methods() {
            let name = method.name();
            println!("method: {}", name);
        }
    }

    //windows_metadata::File::new(r#"D:\git\windows-rs\crates\libs\metadata\default\windows.winmd"#);
}
