use windows_sys::{core::*, Win32::System::Performance::*};

fn main() {
    unsafe {
        let mut query = 0;
        PdhOpenQueryW(std::ptr::null(), 0, &mut query);

        let mut counter = 0;
        PdhAddCounterW(
            query,
            w!("\\Processor(0)\\% Processor Time"),
            0,
            &mut counter,
        );

        loop {
            std::thread::sleep(std::time::Duration::new(1, 0));
            PdhCollectQueryData(query);

            let mut value = std::mem::zeroed();
            if 0 == PdhGetFormattedCounterValue(
                counter,
                PDH_FMT_DOUBLE,
                std::ptr::null_mut(),
                &mut value,
            ) {
                println!("{:.2}", value.Anonymous.doubleValue);
            }
        }
    }
}
