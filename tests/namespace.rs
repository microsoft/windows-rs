// This test ensures that a multi-word namespace "MachineLearning" is properly
// converted to "machine_learning" and imports correctly.

winrt::import!(
    dependencies
        os
    types
        windows::ai::machine_learning::*
);

#[test]
fn namespace() -> winrt::Result<()> {
    use windows::ai::machine_learning::{TensorBoolean, TensorKind};

    let tensor = TensorBoolean::create()?;
    assert!(tensor.tensor_kind()? == TensorKind::Boolean);

    Ok(())
}
