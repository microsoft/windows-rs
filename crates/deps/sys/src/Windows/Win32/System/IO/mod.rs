#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn BindIoCompletionCallback();
    fn CancelIo();
    fn CancelIoEx();
    fn CancelSynchronousIo();
    fn CreateIoCompletionPort();
    fn DeviceIoControl();
    fn GetOverlappedResult();
    fn GetOverlappedResultEx();
    fn GetQueuedCompletionStatus();
    fn GetQueuedCompletionStatusEx();
    fn LPOVERLAPPED_COMPLETION_ROUTINE();
    fn OVERLAPPED();
    fn OVERLAPPED_ENTRY();
    fn PostQueuedCompletionStatus();
}
