#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn BuildCommDCBA();
    fn BuildCommDCBAndTimeoutsA();
    fn BuildCommDCBAndTimeoutsW();
    fn BuildCommDCBW();
    fn ClearCommBreak();
    fn ClearCommError();
    fn CommConfigDialogA();
    fn CommConfigDialogW();
    fn EscapeCommFunction();
    fn GetCommConfig();
    fn GetCommMask();
    fn GetCommModemStatus();
    fn GetCommPorts();
    fn GetCommProperties();
    fn GetCommState();
    fn GetCommTimeouts();
    fn GetDefaultCommConfigA();
    fn GetDefaultCommConfigW();
    fn OpenCommPort();
    fn PurgeComm();
    fn SetCommBreak();
    fn SetCommConfig();
    fn SetCommMask();
    fn SetCommState();
    fn SetCommTimeouts();
    fn SetDefaultCommConfigA();
    fn SetDefaultCommConfigW();
    fn SetupComm();
    fn TransmitCommChar();
    fn WaitCommEvent();
}
