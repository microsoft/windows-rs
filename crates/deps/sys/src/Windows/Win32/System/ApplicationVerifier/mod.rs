#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AVRF_BACKTRACE_INFORMATION();
    fn AVRF_HANDLEOPERATION_ENUMERATE_CALLBACK();
    fn AVRF_HANDLE_OPERATION();
    fn AVRF_HEAPALLOCATION_ENUMERATE_CALLBACK();
    fn AVRF_HEAP_ALLOCATION();
    fn AVRF_MAX_TRACES();
    fn AVRF_RESOURCE_ENUMERATE_CALLBACK();
    fn VERIFIER_ENUM_RESOURCE_FLAGS();
    fn VerifierEnumerateResource();
    fn eAvrfResourceTypes();
    fn eHANDLE_TRACE_OPERATIONS();
    fn eHeapAllocationState();
    fn eHeapEnumerationLevel();
    fn eUserAllocationState();
}
