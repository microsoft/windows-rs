fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let target = std::env::var("TARGET").unwrap();

    let machine = if target.contains("x86_64") {
        "X64"
    } else if target.contains("ARM64") {
        "ARM64"
    } else if target.contains("ARM") {
        "ARM"
    } else if target.contains("i686") {
        "X86"
    } else {
        panic!("Unsupported target");
    };

    let lib = cc::windows_registry::find_tool(&target, "lib.exe").expect("lib.exe not found");

    for path in std::fs::read_dir("def")
        .unwrap()
        .map(|path| path.unwrap().path())
    {
        let mut cmd = lib.to_command();
        let def = path.display();
        println!("cargo:rerun-if-changed={}", def);
        cmd.arg(format!("/def:{}", def));
        cmd.arg(format!("/machine:{}", machine));

        cmd.arg(format!(
            "/out:{}\\{}.lib",
            out_dir,
            path.file_stem().unwrap().to_str().unwrap()
        ));

        cmd.output().unwrap();
    }

    println!("cargo:rustc-link-search={}", out_dir);
}
