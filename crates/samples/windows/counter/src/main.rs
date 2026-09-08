fn main() -> windows::core::Result<()> {
    use windows::{Win32::*, core::*};

    fn check(status: PDH_STATUS) -> Result<()> {
        HRESULT(status.0).ok()
    }

    unsafe {
        let mut query = PDH_HQUERY::default();
        check(PdhOpenQueryW(None, 0, &mut query))?;

        let mut counter = PDH_HCOUNTER::default();
        check(PdhAddEnglishCounterW(
            query,
            w!("\\Processor(_Total)\\% Processor Time"),
            0,
            &mut counter,
        ))?;

        // Rate counters need an initial sample before they can produce a value.
        check(PdhCollectQueryData(query))?;

        for _ in 0..10 {
            std::thread::sleep(std::time::Duration::from_secs(1));
            check(PdhCollectQueryData(query))?;

            let mut value = PDH_FMT_COUNTERVALUE::default();
            check(PdhGetFormattedCounterValue(
                counter,
                PDH_FMT_DOUBLE,
                None,
                &mut value,
            ))?;
            check(PDH_STATUS(value.CStatus as i32))?;
            println!("{:.2}%", value.Anonymous.doubleValue);
        }

        check(PdhCloseQuery(query))
    }
}
