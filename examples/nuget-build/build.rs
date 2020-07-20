winrt::build!(
    dependencies
        nuget: Microsoft.Windows.SDK.Contracts
    types
        windows::foundation::Uri
);

fn main() {
    build();
}
