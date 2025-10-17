use test_sys_fn_ptrs::bindings::Windows::Win32::Networking::WindowsWebServices::*;

#[allow(unused)]
#[allow(unused_assignments)]
#[test]
fn test_binding() {
    let mut f: WebAuthNAuthenticatorMakeCredential = WebAuthNAuthenticatorMakeCredential;
    f = test_sys_fn_ptrs::bind_getprocaddr_make_credential().unwrap();
    f = test_sys_fn_ptrs::bind_linked_make_credential();
}
