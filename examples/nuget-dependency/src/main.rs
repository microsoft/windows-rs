winrt::import!(
    dependencies
        nuget: Microsoft.Windows.SDK.Contracts
    modules
        "windows.foundation"
);
use windows::foundation::PropertyValue;

fn main() {
    let _value = PropertyValue::create_boolean(true).unwrap();
}
