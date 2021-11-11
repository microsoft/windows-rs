#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CallNamedPipeA();
    fn CallNamedPipeW();
    fn ConnectNamedPipe();
    fn CreateNamedPipeA();
    fn CreateNamedPipeW();
    fn CreatePipe();
    fn DisconnectNamedPipe();
    fn GetNamedPipeClientComputerNameA();
    fn GetNamedPipeClientComputerNameW();
    fn GetNamedPipeClientProcessId();
    fn GetNamedPipeClientSessionId();
    fn GetNamedPipeHandleStateA();
    fn GetNamedPipeHandleStateW();
    fn GetNamedPipeInfo();
    fn GetNamedPipeServerProcessId();
    fn GetNamedPipeServerSessionId();
    fn ImpersonateNamedPipeClient();
    fn NAMED_PIPE_MODE();
    fn NMPWAIT_NOWAIT();
    fn NMPWAIT_USE_DEFAULT_WAIT();
    fn NMPWAIT_WAIT_FOREVER();
    fn PIPE_UNLIMITED_INSTANCES();
    fn PeekNamedPipe();
    fn SetNamedPipeHandleState();
    fn TransactNamedPipe();
    fn WaitNamedPipeA();
    fn WaitNamedPipeW();
}
