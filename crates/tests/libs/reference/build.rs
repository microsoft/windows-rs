// Generates `src/bindings.rs` with the regular (non-minimal) `IPropertyValue`
// bindings so the ABI conformance test in `tests/stock_reference.rs` exercises
// the stock `IReference<T>` implementation against bindgen-emitted safe wrappers
// (`Type()`, `IsNumericScalar()`, the typed `Get*()` accessors, …) rather than a
// hand-rolled vtable — guarding against subtle ABI drift in either direction.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    windows_bindgen::bindgen([
        "--in",
        "default",
        "--out",
        "src/bindings.rs",
        "--filter",
        "Windows.Foundation.DateTime",
        "Windows.Foundation.IPropertyValue",
        "Windows.Foundation.Point",
        "Windows.Foundation.PropertyType",
        "Windows.Foundation.Rect",
        "Windows.Foundation.Size",
        "Windows.Foundation.TimeSpan",
        "Windows.Foundation.Numerics.Vector2",
        "--flat",
    ])
    .unwrap();
}
