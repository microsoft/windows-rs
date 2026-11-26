fn main() -> windows::core::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    use windows::{
        Win32::Networking::BackgroundIntelligentTransferService::*, Win32::System::Com::*, core::*,
    };

    #[implement(IBackgroundCopyCallback)]
    struct Callback;

    impl IBackgroundCopyCallback_Impl for Callback_Impl {
        fn JobTransferred(&self, job: Ref<IBackgroundCopyJob>) -> Result<()> {
            let job = job.unwrap();
            unsafe { job.Complete()? };
            println!("done");
            std::process::exit(0);
        }

        fn JobError(
            &self,
            job: Ref<IBackgroundCopyJob>,
            error: Ref<IBackgroundCopyError>,
        ) -> Result<()> {
            let job = job.unwrap();
            let error = error.unwrap();
            unsafe {
                job.Cancel()?;
                println!("{}", error.GetErrorDescription(0)?.display());
            }
            std::process::exit(0);
        }

        fn JobModification(&self, _: Ref<IBackgroundCopyJob>, _: u32) -> Result<()> {
            Ok(())
        }
    }

    unsafe extern "C" {
        pub fn getchar() -> i32;
    }

    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;

        let manager: IBackgroundCopyManager =
            CoCreateInstance(&BackgroundCopyManager, None, CLSCTX_LOCAL_SERVER)?;

        let mut job = None;
        manager.CreateJob(
            w!("sample"),
            BG_JOB_TYPE_DOWNLOAD,
            &mut Default::default(),
            &mut job,
        )?;
        let job = job.unwrap();

        let destination = std::env::temp_dir().join("windows-rs-bits.png");
        let destination_wide: Vec<u16> = destination
            .as_os_str()
            .encode_wide()
            .chain(Some(0))
            .collect();
        job.AddFile(
            w!(
                "https://raw.githubusercontent.com/microsoft/windows-rs/master/crates/samples/canvas/bitmap/sample.png"
            ),
            PCWSTR(destination_wide.as_ptr()),
        )?;

        let callback: IBackgroundCopyCallback = Callback.into();
        job.SetNotifyInterface(&callback)?;
        job.SetNotifyFlags(BG_NOTIFY_JOB_TRANSFERRED | BG_NOTIFY_JOB_ERROR)?;

        job.Resume()?;
        println!("downloading to {}...", destination.display());

        getchar();
        job.Cancel()?;
        println!("canceled");
        Ok(())
    }
}
