// Dogfood the windows-csharp generator: produce the projected `Bench.cs` from the same
// bench.winmd the Rust, CsWinRT, and C++/WinRT consumers use. The generated file is committed
// so the csproj can compile it; regenerating on build keeps it in sync with the winmd.
//
// The WinRT `Windows.Foundation.winmd` is supplied alongside the component winmd so the generator
// can resolve `IWidget::Items`'s `IVector<i32>` return and emit the generic `IVector<T>` projection
// (its parameterized IID is derived from the open `IVector`1` metadata).
fn main() {
    println!("cargo:rerun-if-changed=../component/bench.winmd");

    windows_csharp::builder()
        .input("../component/bench.winmd")
        .input("../../../libs/bindgen/default")
        .input(r"C:\Windows\System32\WinMetadata\Windows.Foundation.winmd")
        .filter("Bench")
        .output("Bench.cs")
        .write()
        .unwrap();
}
