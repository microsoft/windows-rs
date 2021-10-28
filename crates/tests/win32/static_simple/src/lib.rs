// Remove target_pointer_width when upstream metadata generator supports other targets
#![cfg(all(windows, target_pointer_width = "64", target_env = "msvc"))]

windows::runtime::include_bindings!();
