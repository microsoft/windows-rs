winrt::build!(
    dependencies
        nuget: Microsoft.Windows.SDK.Contracts
    types
        windows::ui::Colors
);

fn main() {
    build();
}
