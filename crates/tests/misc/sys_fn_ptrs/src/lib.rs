// Tests that --sys + --sys-fn-ptr bindings work and generate a function pointer for Win32 APIs.

pub mod bindings;

use bindings::Windows::Win32::System::LibraryLoader::*;

use bindings::Windows::Win32::Networking::WindowsWebServices::WebAuthNAuthenticatorMakeCredential;

unsafe fn delay_load<T>(library: bindings::PCSTR, function: bindings::PCSTR) -> Option<T> {
    unsafe {
        let library = LoadLibraryExA(
            library,
            std::ptr::null_mut(),
            LOAD_LIBRARY_SEARCH_DEFAULT_DIRS,
        );

        if library.is_null() {
            return None;
        };

        let address = GetProcAddress(library, function);

        if address.is_some() {
            return Some(std::mem::transmute_copy(&address));
        }

        // In real code you'd need to manage your library lifetimes by calling FreeLibrary when you're done with your delay_load
        None
    }
}

#[allow(unused)]
pub fn bind_getprocaddr_make_credential() -> Option<WebAuthNAuthenticatorMakeCredential> {
    unsafe {
        delay_load(
            windows_strings::s!("webauthn.dll").as_ptr(),
            windows_strings::s!("WebAuthNAuthenticatorMakeCredential").as_ptr(),
        )
    }
}

#[allow(unused)]
pub fn bind_linked_make_credential() -> WebAuthNAuthenticatorMakeCredential {
    bindings::Windows::Win32::Networking::WindowsWebServices::WebAuthNAuthenticatorMakeCredential
}
