use super::*;
use std::sync::OnceLock;

/// Returns the process-wide shared DirectWrite factory.
pub(crate) fn factory() -> &'static IDWriteFactory {
    // The shared factory is thread-safe, but the metadata does not mark `IDWriteFactory`
    // `[agile]`. Wrap it for the process-wide `OnceLock`.
    struct Shared(IDWriteFactory);
    unsafe impl Send for Shared {}
    unsafe impl Sync for Shared {}

    static SHARED: OnceLock<Shared> = OnceLock::new();

    &SHARED
        .get_or_init(|| {
            let mut factory: Option<IDWriteFactory> = None;
            unsafe {
                DWriteCreateFactory(
                    DWRITE_FACTORY_TYPE_SHARED,
                    &IDWriteFactory::IID,
                    &mut factory as *mut _ as *mut _,
                )
                .unwrap();
            }
            Shared(factory.unwrap())
        })
        .0
}
