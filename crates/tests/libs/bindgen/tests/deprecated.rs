#[allow(deprecated)]
use test_bindgen::deprecated_class::*;
#[allow(deprecated)]
use test_bindgen::deprecated_enum::*;
#[allow(deprecated)]
use test_bindgen::deprecated_struct::*;

#[test]
fn deprecated_class_methods_exist() {
    // Verify deprecated class methods are generated and callable (they will fail at runtime
    // without the actual WinRT server, but the fact that this compiles proves the bindings work)
    #[allow(deprecated)]
    let _email_fn: fn() -> windows_core::Result<windows_core::HSTRING> = KnownContactField::Email;
    #[allow(deprecated)]
    let _phone_fn: fn() -> windows_core::Result<windows_core::HSTRING> =
        KnownContactField::PhoneNumber;
}

#[test]
fn deprecated_enum_values_exist() {
    #[allow(deprecated)]
    {
        assert_eq!(PlayToConnectionState::Disconnected.0, 0);
        assert_eq!(PlayToConnectionState::Connected.0, 1);
        assert_eq!(PlayToConnectionState::Rendering.0, 2);
    }
}

#[test]
fn deprecated_struct_has_methods() {
    // PlayToSourceSelectedEventArgs is a deprecated class with deprecated methods.
    // Verify the type and its methods exist by checking we can name them.
    #[allow(deprecated)]
    fn _check(obj: &PlayToSourceSelectedEventArgs) {
        let _ = obj.FriendlyName();
        let _ = obj.SupportsImage();
        let _ = obj.SupportsAudio();
    }
}
