// This test ensures that a multi-word namespace "MachineLearning" is properly
// converted to "machine_learning" and imports correctly.

#[test]
fn namespace() -> windows::Result<()> {
    use test_winrt::Windows::AI::MachineLearning::{TensorBoolean, TensorKind};

    let tensor = TensorBoolean::Create()?;
    assert!(tensor.TensorKind()? == TensorKind::Boolean);

    Ok(())
}
