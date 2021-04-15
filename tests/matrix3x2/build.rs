fn main() {
    windows::build!(
        // The Windows crate manually injects a functions needed to implement Matrix3x2.
        // This test validates this is included.
        Windows::Foundation::Numerics::Matrix3x2
    );
}
