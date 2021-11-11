#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CreateToolhelp32Snapshot();
    fn Heap32First();
    fn Heap32ListFirst();
    fn Heap32ListNext();
    fn Heap32Next();
    fn Module32First();
    fn Module32FirstW();
    fn Module32Next();
    fn Module32NextW();
    fn Process32First();
    fn Process32FirstW();
    fn Process32Next();
    fn Process32NextW();
    fn Thread32First();
    fn Thread32Next();
    fn Toolhelp32ReadProcessMemory();
}
