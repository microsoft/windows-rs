// TODO: test_win32_return only works on x64 due to a Rust x86 linker bug
#![cfg(target_pointer_width = "64")]

windows::runtime::include_bindings!();
