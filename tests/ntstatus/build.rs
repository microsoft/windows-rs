fn main() {
    windows::build! {
        Windows::Win32::Foundation::{STATUS_NOT_FOUND, STATUS_INVALID_SIGNATURE, STATUS_SUCCESS},
        Windows::Win32::Security::Cryptography::Core::{
            BCryptGenRandom, BCryptOpenAlgorithmProvider,
        },
    };
}
