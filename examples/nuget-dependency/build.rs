use winrt::Builder;

fn main() {
    Builder::default()
        .insert_namespaces(&["windows.foundation"])
        .insert_nuget(&["Microsoft.Windows.SDK.Contracts"])
        .build()
        .unwrap();
}
