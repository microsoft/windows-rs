#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CREATE_TOOLHELP_SNAPSHOT_FLAGS();
    fn CreateToolhelp32Snapshot();
    fn HEAPENTRY32();
    fn HEAPENTRY32_FLAGS();
    fn HEAPLIST32();
    fn HF32_DEFAULT();
    fn HF32_SHARED();
    fn Heap32First();
    fn Heap32ListFirst();
    fn Heap32ListNext();
    fn Heap32Next();
    fn MAX_MODULE_NAME32();
    fn MODULEENTRY32();
    fn MODULEENTRY32W();
    fn Module32First();
    fn Module32FirstW();
    fn Module32Next();
    fn Module32NextW();
    fn PROCESSENTRY32();
    fn PROCESSENTRY32W();
    fn Process32First();
    fn Process32FirstW();
    fn Process32Next();
    fn Process32NextW();
    fn THREADENTRY32();
    fn Thread32First();
    fn Thread32Next();
    fn Toolhelp32ReadProcessMemory();
}
