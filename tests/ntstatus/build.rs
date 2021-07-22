fn main() {
    windows::build! {
        Windows::Win32::Foundation::STATUS_NOT_FOUND,
        Windows::Win32::Security::Cryptography::Core::{
            BCryptGenRandom, BCryptOpenAlgorithmProvider,
        },
    };
}
