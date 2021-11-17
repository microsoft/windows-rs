fn main() {
    windows::core::build_legacy! {
        Windows::Win32::Foundation::{STATUS_INVALID_SIGNATURE, STATUS_NOT_FOUND, STATUS_SUCCESS},
        Windows::Win32::Security::Cryptography::{BCryptGenRandom, BCryptOpenAlgorithmProvider},
    };
}
