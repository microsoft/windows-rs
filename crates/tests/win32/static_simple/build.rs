fn main() {
    // Remove target_arch when upstream metadata generator supports other targets
    #[cfg(all(windows, target_arch = "x86_64", target_env = "msvc"))]
    windows::runtime::build! {
        StaticComponent::Win32::Simple::*,
    };
}
