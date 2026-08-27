fn main() {
    if std::env::var_os("CARGO_FEATURE_SELF_CONTAINED").is_some() {
        windows_reactor_setup::as_self_contained();
    } else {
        windows_reactor_setup::as_framework_dependent();
    }
}
