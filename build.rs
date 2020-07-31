winrt_macros::build!(
    foundation
    dependencies
        nuget: Microsoft.Windows.SDK.Contracts
    types
);

fn main() {
    build();
}
