winrt::import!(
    dependencies
        os
    types
        windows::globalization::ICurrencyIdentifiersStatics
);

// Usage of method named `try` when `ICurrencyIdentifiersStatics` is generated
// This tests that it is escaped
#[test]
fn escaped() -> winrt::Result<()> {
    Ok(())
}
