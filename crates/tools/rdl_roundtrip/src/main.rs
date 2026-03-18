use windows_rdl::*;

fn roundtrip(winmd: &str, rdl: &str, filter: &[&str]) {
    let mut w = writer();
    w.input("crates/libs/bindgen/default").output(rdl).split();
    for f in filter {
        w.filter(f);
    }
    w.write().unwrap();

    reader()
        .input(rdl)
        .reference("crates/libs/bindgen/default")
        .output(winmd)
        .write()
        .unwrap();
}

fn main() {
    let time = std::time::Instant::now();

    roundtrip(
        "crates/libs/bindgen/default/Windows.winmd",
        "target/rdl/Windows",
        &["Windows", "!Windows.Win32", "!Windows.Wdk"],
    );

    roundtrip(
        "crates/libs/bindgen/default/Windows.Win32.winmd",
        "target/rdl/Windows.Win32",
        &["Windows.Win32"],
    );

    roundtrip(
        "crates/libs/bindgen/default/Windows.Wdk.winmd",
        "target/rdl/Windows.Wdk",
        &["Windows.Wdk"],
    );

    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}
