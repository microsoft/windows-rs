fn main() {
    // This test validates that IMap pulls in its dependencies including IIterator and IKeyValuePair.
    // https://github.com/microsoft/windows-rs/issues/772
    windows::build! {
        Windows::Foundation::Collections::IMap,
    };
}
