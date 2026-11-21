// Generates `src/bindings.rs` with the full `IPropertyValue`
// bindings so the ABI conformance test in `tests/stock_reference.rs` exercises
// the stock `IReference<T>` implementation against bindgen-emitted safe wrappers
// (`Type()`, `IsNumericScalar()`, and typed `Get*()` accessors) rather than a
// hand-rolled vtable, guarding against ABI drift in either direction.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    windows_bindgen::bindgen([
        "--in",
        "default",
        "--out",
        "src/bindings.rs",
        "--full",
        "--filter",
        "Windows.Foundation.DateTime",
        "Windows.Foundation.IPropertyValue",
        "Windows.Foundation.Point",
        "Windows.Foundation.PropertyType",
        "Windows.Foundation.Rect",
        "Windows.Foundation.Size",
        "Windows.Foundation.TimeSpan",
        "Windows.Foundation.Numerics.Vector2",
    ]);
}
