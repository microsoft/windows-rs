#[cfg(target_pointer_width = "32")]
#[cfg(feature = "Win32_Foundation")]
pub use GetWindowLongA as GetWindowLongPtrA;
#[cfg(target_pointer_width = "32")]
#[cfg(feature = "Win32_Foundation")]
pub use GetWindowLongW as GetWindowLongPtrW;
#[cfg(target_pointer_width = "32")]
#[cfg(feature = "Win32_Foundation")]
pub use SetWindowLongA as SetWindowLongPtrA;
#[cfg(target_pointer_width = "32")]
#[cfg(feature = "Win32_Foundation")]
pub use SetWindowLongW as SetWindowLongPtrW;
