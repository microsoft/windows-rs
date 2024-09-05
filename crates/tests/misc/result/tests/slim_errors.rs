use windows_result::*;

#[test]
fn show_sizes() {
    use core::mem::size_of;

    static_assertions::assert_impl_all!(Error: Send, Sync);

    if cfg!(windows_slim_errors) {
        assert_eq!(size_of::<Result<()>>(), size_of::<HRESULT>());
    }
}
