use windows_metadata::{HasAttributes, reader};

#[test]
fn must_use_roundtrip() {
    let source = r#"
        #[win32] mod Test {
            #[must_use] #[library("test.dll")] extern fn Read() -> i32;
            #[must_use] extern fn Callback(value: i32) -> i32;
            #[no_guid] interface Object {
                #[must_use] fn Read(&self) -> i32;
            }
        }
    "#;
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let output = std::path::Path::new(env!("OUT_DIR"));
    let mut previous = None;
    for (id, source) in [
        source.to_string(),
        source.replace("must_use", "Windows::Win32::Metadata::MustUse"),
    ]
    .iter()
    .enumerate()
    {
        let winmd = output.join(format!("must-use-{id}.winmd"));
        let rdl = winmd.with_extension("rdl");
        windows_rdl::reader()
            .input(root.join("metadata\\metadata.rdl"))
            .input_text(source)
            .output(&winmd)
            .write()
            .unwrap();
        let index = reader::Index::read(&winmd).unwrap();
        for owner in ["Apis", "Object"] {
            let method = index.expect("Test", owner).methods().next().unwrap();
            assert!(
                method
                    .find_attribute("MustUseAttribute")
                    .unwrap()
                    .value()
                    .is_empty()
            );
        }
        assert!(
            index
                .expect("Test", "Callback")
                .find_attribute("MustUseAttribute")
                .unwrap()
                .value()
                .is_empty()
        );
        windows_rdl::writer()
            .input(&winmd)
            .filter("Test")
            .output(&rdl)
            .write()
            .unwrap();
        let text = std::fs::read_to_string(rdl).unwrap();
        assert_eq!(text.matches("#[must_use]").count(), 3);
        if let Some(previous) = &previous {
            assert_eq!(&text, previous);
        }
        previous = Some(text);
    }
    for spelling in ["must_use(\"reason\")", "must_use = \"reason\""] {
        assert!(
            windows_rdl::reader()
                .input(root.join("metadata\\metadata.rdl"))
                .input_text(&source.replace("must_use", spelling))
                .output(output.join("invalid-must-use.winmd"))
                .write()
                .is_err()
        );
    }
}
