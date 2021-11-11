#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn WSManCloseCommand();
    fn WSManCloseOperation();
    fn WSManCloseSession();
    fn WSManCloseShell();
    fn WSManConnectShell();
    fn WSManConnectShellCommand();
    fn WSManCreateSession();
    fn WSManCreateShell();
    fn WSManCreateShellEx();
    fn WSManDeinitialize();
    fn WSManDisconnectShell();
    fn WSManGetErrorMessage();
    fn WSManGetSessionOptionAsDword();
    fn WSManGetSessionOptionAsString();
    fn WSManInitialize();
    fn WSManPluginAuthzOperationComplete();
    fn WSManPluginAuthzQueryQuotaComplete();
    fn WSManPluginAuthzUserComplete();
    fn WSManPluginFreeRequestDetails();
    fn WSManPluginGetConfiguration();
    fn WSManPluginGetOperationParameters();
    fn WSManPluginOperationComplete();
    fn WSManPluginReceiveResult();
    fn WSManPluginReportCompletion();
    fn WSManPluginReportContext();
    fn WSManReceiveShellOutput();
    fn WSManReconnectShell();
    fn WSManReconnectShellCommand();
    fn WSManRunShellCommand();
    fn WSManRunShellCommandEx();
    fn WSManSendShellInput();
    fn WSManSetSessionOption();
    fn WSManSignalShell();
}
