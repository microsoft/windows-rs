// This test ensures that a multi-word namespace "MachineLearning" is properly
// converted to "machine_learning" and imports correctly.

mod generated {
    winrt::import!(
        dependencies
            os
        types
            windows::foundation::*
            windows::ai::machine_learning::*
    );
}

#[test]
fn namespace() -> winrt::Result<()> {
    use generated::windows::ai::machine_learning::{TensorBoolean, TensorKind};

    let tensor = TensorBoolean::create()?;
    assert!(tensor.tensor_kind()? == TensorKind::Boolean);

    Ok(())
}
