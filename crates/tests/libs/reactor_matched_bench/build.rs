fn main() {
    if std::env::var_os("CARGO_FEATURE_LIVE").is_some() {
        windows_reactor_setup::as_framework_dependent();
    }
}
