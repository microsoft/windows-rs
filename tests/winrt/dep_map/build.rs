fn main() {
    // This test validates that IMap pulls in its dependencies including IIterator and IKeyValuePair.
    // This is a particularly interesting test because IMap only mentions IKeyValuePair indirectly
    // through its interface requirement on the specialization of IIterable.
    windows::build! {
        Windows::Foundation::Collections::IMap,
    };
}
