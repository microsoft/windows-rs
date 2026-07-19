fn main() -> windows::core::Result<()> {
    use windows::{Win32::*, core::*};

    #[implement(IBackgroundCopyCallback)]
    struct Callback;

    impl IBackgroundCopyCallback_Impl for Callback_Impl {
        fn JobTransferred(&self, job: Ref<IBackgroundCopyJob>) -> Result<()> {
            let job = job.unwrap();
            unsafe { job.Complete().ok()? };
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
                job.Cancel().ok()?;
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
        CoInitializeEx(None, COINIT_MULTITHREADED as u32).ok()?;

        let manager: IBackgroundCopyManager =
            CoCreateInstance(&BackgroundCopyManager, None, CLSCTX_LOCAL_SERVER)?;

        let mut job = None;

        manager
            .CreateJob(
                w!("sample"),
                BG_JOB_TYPE_DOWNLOAD,
                &mut Default::default(),
                &mut job,
            )
            .ok()?;

        let job = job.unwrap();
        job.AddFile(w!("https://kennykerr.ca/favicon.svg"), w!("D:\\rust.svg"))
            .ok()?;

        let callback: IBackgroundCopyCallback = Callback.into();
        job.SetNotifyInterface(&callback).ok()?;
        job.SetNotifyFlags(BG_NOTIFY_JOB_TRANSFERRED | BG_NOTIFY_JOB_ERROR)
            .ok()?;

        job.Resume().ok()?;
        println!("downloading...");

        getchar();
        job.Cancel().ok()?;
        println!("canceled");
        Ok(())
    }
}
