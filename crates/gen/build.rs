fn main() {
    fn copy(source: &::std::path::Path, destination: &mut ::std::path::PathBuf) {
        if let ::std::result::Result::Ok(files) = ::std::fs::read_dir(source) {
            for file in files.filter_map(|file| file.ok())  {
                if let ::std::result::Result::Ok(file_type) = file.file_type() {
                    // TODO: make this recursive?
                    if file_type.is_file() {
                        let path = file.path();
                        if let ::std::option::Option::Some(filename) = path.file_name() {
                            std::fs::create_dir_all(&destination);
                            destination.push(filename);

                            {
                                use std::io::prelude::*;
                                let mut file = std::fs::OpenOptions::new()
                                .append(true)
                                .create(true)
                                .open("C:\\git\\vars.txt")
                                .unwrap();
                            
                                writeln!(file, "gen copy: {:?} = {:?}", path, destination).unwrap();
                            }

                            let _ = ::std::fs::copy(path, &destination);
                            destination.pop();
                        }
                    }
                }
            }
        }
    }

    let mut source : ::std::path::PathBuf = ::std::env::var("CARGO_MANIFEST_DIR").expect("No `CARGO_MANIFEST_DIR` env variable set").into();
    source.push(".windows");
    source.push("winmd");

    let mut destination : ::std::path::PathBuf = ::std::env::var("OUT_DIR").expect("No `OUT_DIR` env variable set").into();
    loop {
        destination.pop();
        destination.push("Cargo.toml");

        if destination.exists() {
            break;
        }

        destination.pop();
    }

    destination.pop();
    destination.push("target");
    destination.push(".windows");
    destination.push("winmd");

    copy(&source, &mut destination);
}
