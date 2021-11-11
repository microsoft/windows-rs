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
    fn PeekNamedPipe();
    fn SetNamedPipeHandleState();
    fn TransactNamedPipe();
    fn WaitNamedPipeA();
    fn WaitNamedPipeW();
}
