// This test ensures that a multi-word namespace "MachineLearning" is properly
// converted to "machine_learning" and imports correctly.

#[test]
fn namespace() -> windows::Result<()> {
    use tests::windows::ai::machine_learning::{TensorBoolean, TensorKind};

    let tensor = TensorBoolean::create()?;
    assert!(tensor.tensor_kind()? == TensorKind::Boolean);

    Ok(())
}
