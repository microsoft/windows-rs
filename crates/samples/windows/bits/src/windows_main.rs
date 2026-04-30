use windows::{
    core::*, Win32::Networking::BackgroundIntelligentTransferService::*, Win32::System::Com::*,
};

fn main() -> Result<()> {
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
        job.AddFile(w!("https://kennykerr.ca/favicon.svg"), w!("D:\\rust.svg"))?;

        let callback: IBackgroundCopyCallback = Callback.into();
        job.SetNotifyInterface(&callback)?;
        job.SetNotifyFlags(BG_NOTIFY_JOB_TRANSFERRED | BG_NOTIFY_JOB_ERROR)?;

        job.Resume()?;
        println!("downloading...");

        getchar();
        job.Cancel()?;
        println!("canceled");
        Ok(())
    }
}

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
