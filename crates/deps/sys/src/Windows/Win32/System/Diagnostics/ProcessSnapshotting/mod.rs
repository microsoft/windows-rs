#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn HPSS();
    fn HPSSWALK();
    fn PSS_ALLOCATOR();
    fn PSS_AUXILIARY_PAGES_INFORMATION();
    fn PSS_AUXILIARY_PAGE_ENTRY();
    fn PSS_CAPTURE_FLAGS();
    fn PSS_DUPLICATE_FLAGS();
    fn PSS_HANDLE_ENTRY();
    fn PSS_HANDLE_FLAGS();
    fn PSS_HANDLE_INFORMATION();
    fn PSS_HANDLE_TRACE_INFORMATION();
    fn PSS_OBJECT_TYPE();
    fn PSS_PERFORMANCE_COUNTERS();
    fn PSS_PERF_RESOLUTION();
    fn PSS_PROCESS_FLAGS();
    fn PSS_PROCESS_INFORMATION();
    fn PSS_QUERY_INFORMATION_CLASS();
    fn PSS_THREAD_ENTRY();
    fn PSS_THREAD_FLAGS();
    fn PSS_THREAD_INFORMATION();
    fn PSS_VA_CLONE_INFORMATION();
    fn PSS_VA_SPACE_ENTRY();
    fn PSS_VA_SPACE_INFORMATION();
    fn PSS_WALK_INFORMATION_CLASS();
    fn PssCaptureSnapshot();
    fn PssDuplicateSnapshot();
    fn PssFreeSnapshot();
    fn PssQuerySnapshot();
    fn PssWalkMarkerCreate();
    fn PssWalkMarkerFree();
    fn PssWalkMarkerGetPosition();
    fn PssWalkMarkerSeekToBeginning();
    fn PssWalkMarkerSetPosition();
    fn PssWalkSnapshot();
}
