#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn WMCreateBackupRestorer();
    fn WMCreateEditor();
    fn WMCreateIndexer();
    fn WMCreateProfileManager();
    fn WMCreateReader();
    fn WMCreateSyncReader();
    fn WMCreateWriter();
    fn WMCreateWriterFileSink();
    fn WMCreateWriterNetworkSink();
    fn WMCreateWriterPushSink();
    fn WMIsContentProtected();
}
