#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn RtlDrainNonVolatileFlush();
    fn RtlFillNonVolatileMemory();
    fn RtlFlushNonVolatileMemory();
    fn RtlFlushNonVolatileMemoryRanges();
    fn RtlFreeNonVolatileToken();
    fn RtlGetNonVolatileToken();
    fn RtlWriteNonVolatileMemory();
}
