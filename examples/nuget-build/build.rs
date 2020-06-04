winrt::build!(
    dependencies
        nuget: Microsoft.AI.MachineLearning
        nuget: Microsoft.Windows.SDK.Contracts
    types
        microsoft::ai::machine_learning::*
        windows::foundation::Uri
);

use std::process::Command;

fn main() {
    build();
}
