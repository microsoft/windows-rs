winrt::import!(
    dependencies
        os
        nuget: Microsoft.AI.MachineLearning
    types
        microsoft::ai::machine_learning::*
);

fn main() -> winrt::Result<()> {
    use microsoft::ai::machine_learning::*;

    let tensor = TensorBoolean::create()?;
    assert!(tensor.tensor_kind()? == TensorKind::Boolean);
    println!("Tensor: {:?}", tensor);

    Ok(())
}
