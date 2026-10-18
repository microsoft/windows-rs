#![allow(non_snake_case, non_camel_case_types, dead_code)]
#![cfg_attr(named_import_probe, no_std)]
#![cfg_attr(named_import_probe, no_main)]

include!("bindings.rs");

fn check_calls() {
    unsafe {
        let heap = GetProcessHeap();
        assert!(!heap.is_null());
        let block = HeapAlloc(heap, 0, 16).cast::<u8>();
        assert!(!block.is_null());
        for index in 0..16 {
            block.add(index).write(index as u8);
        }
        let resized = HeapReAlloc(heap, 0, block.cast(), 64).cast::<u8>();
        assert!(!resized.is_null());
        for index in 0..16 {
            assert_eq!(resized.add(index).read(), index as u8);
        }
        resized.add(63).write(42);
        assert_ne!(HeapFree(heap, 0, resized.cast()), 0);

        let functions: [(GlobalAlloc, GlobalReAlloc, GlobalFree); 2] = [
            (GlobalAlloc, GlobalReAlloc, GlobalFree),
            (LocalAlloc, LocalReAlloc, LocalFree),
        ];
        for (allocate, resize, free) in functions {
            let block = allocate(0, 16);
            assert!(!block.is_null());
            block.cast::<u8>().write(37);
            // GMEM_MOVEABLE / LMEM_MOVEABLE permit relocation of the fixed block.
            let resized = resize(block, 8, 0x0002);
            assert!(!resized.is_null());
            assert_eq!(resized.cast::<u8>().read(), 37);
            assert!(free(resized).is_null());
        }
    }
}

#[cfg(not(named_import_probe))]
fn main() {
    check_calls();
}

#[cfg(named_import_probe)]
windows_link::link!("kernel32.dll" "system" fn ExitProcess(code: u32) -> !);

#[cfg(named_import_probe)]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    unsafe { ExitProcess(1) }
}

#[cfg(named_import_probe)]
#[unsafe(no_mangle)]
extern "C" fn mainCRTStartup() -> ! {
    check_calls();
    unsafe { ExitProcess(0) }
}
