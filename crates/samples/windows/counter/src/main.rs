use windows::{core::*, Win32::System::Performance::*};

fn main() {
    unsafe {
        let mut query = PDH_HQUERY::default();
        PdhOpenQueryW(None, 0, &mut query);

        let mut counter = PDH_HCOUNTER::default();
        PdhAddCounterW(
            query,
            w!("\\Processor(0)\\% Processor Time"),
            0,
            &mut counter,
        );

        loop {
            std::thread::sleep(std::time::Duration::new(1, 0));
            PdhCollectQueryData(query);

            let mut value = Default::default();
            if 0 == PdhGetFormattedCounterValue(counter, PDH_FMT_DOUBLE, None, &mut value) {
                println!("{:.2}", value.Anonymous.doubleValue);
            }
        }
    }
}
