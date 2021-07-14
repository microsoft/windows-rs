use bindings::Windows::Win32::Gaming::HasExpandedResources;

#[test]
fn test() {
    unsafe {
        HasExpandedResources().unwrap();
    }
}
