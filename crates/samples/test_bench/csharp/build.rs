// Dogfood the windows-csharp generator: produce the projected `Bench.cs` from the same
// bench.winmd the Rust, CsWinRT, and C++/WinRT consumers use. The generated file is committed
// so the csproj can compile it; regenerating on build keeps it in sync with the winmd.
//
fn main() {
    println!("cargo:rerun-if-changed=../component/bench.winmd");

    windows_csharp::builder()
        .input("../component/bench.winmd")
        .input_default()
        .filter("Bench")
        .output("Bench.cs")
        .write()
        .unwrap();
}
