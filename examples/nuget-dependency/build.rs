use winrt::Builder;

fn main() {
    Builder::default()
        .insert_namespaces(&["windows.foundation"])
        .build()
        .unwrap();
}
