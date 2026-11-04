use super::*;

#[test]
fn image_decode_failure_and_recovery() -> Result<()> {
    process::run("image", |ui| {
        ui.name("events", "Opened 1; failed 0")?;
        ui.invoke("invalid")?;
        ui.name("events", "Opened 1; failed 1")?;
        ui.invoke("valid")?;
        ui.name("events", "Opened 2; failed 1")?;
        ui.name_stays("events", "Opened 2; failed 1", Duration::from_millis(250))
    })
}

#[test]
fn svg_images_load_on_live_and_mounting_elements() -> Result<()> {
    process::run("image", |ui| {
        ui.name("events", "Opened 1; failed 0")?;
        ui.invoke("svg")?;
        ui.name("events", "Opened 3; failed 0")?;
        ui.name_stays("events", "Opened 3; failed 0", Duration::from_millis(250))
    })
}

#[test]
fn bitmap_thumbnails_do_not_retain_full_sized_decodes() -> Result<()> {
    process::run("image-bitmap", |ui| {
        ui.name("events", "Opened 0")?;
        let before = private_bytes(ui)?;
        for batch in 0..3 {
            ui.invoke("load")?;
            ui.name("events", &format!("Opened {}", batch * 16 + 8))?;
            let after = private_bytes(ui)?;
            let growth = after.saturating_sub(before);
            eprintln!("bitmap batch={batch} before={before} after={after} growth={growth}");
            // Eight full 2048x2048 BGRA decodes need 128 MiB; 48-DIP thumbnails do not.
            assert!(
                growth < 64 * 1024 * 1024,
                "full-sized bitmap decodes: {growth}"
            );
            ui.invoke("remove")?;
            ui.gone("bitmap-0")?;
            ui.invoke("probe")?;
            ui.name("sample", &format!("Sample {}; bytes 8", batch * 2 + 1))?;
            ui.invoke("remount")?;
            ui.name("events", &format!("Opened {}", (batch + 1) * 16))?;
            ui.invoke("release")?;
            ui.gone("bitmap-0")?;
            ui.invoke("probe")?;
            ui.name("sample", &format!("Sample {}; bytes 0", (batch + 1) * 2))?;
        }
        Ok(())
    })
}

#[test]
fn transparent_bitmaps_still_report_load_completion() -> Result<()> {
    process::run("image-transparent", |ui| {
        ui.name("events", "Opened 1; failed 0")?;
        ui.invoke("invalid")?;
        ui.name("events", "Opened 1; failed 1")?;
        ui.invoke("valid")?;
        ui.name("events", "Opened 2; failed 1")
    })
}

#[test]
fn shared_bitmap_bytes_resize_and_remount_without_blurring() -> Result<()> {
    // The PNG's 16-pixel stripes lose contrast at thumbnail size but resolve at the larger size.
    process::run("image-bitmap", |ui| {
        ui.invoke("load")?;
        ui.name("events", "Opened 8")?;
        let initial = unsafe { ui.element("bitmap-0")?.CurrentBoundingRectangle()? };
        ui.invoke("remove")?;
        ui.gone("bitmap-0")?;
        ui.invoke("share")?;
        ui.invoke("remount")?;
        ui.name("events", "Opened 16")?;
        ui.invoke("resize")?;
        sharp_bitmap(ui, (initial.right - initial.left) * 3)?;
        ui.invoke("remove")?;
        ui.gone("bitmap-0")?;
        ui.invoke("remount")?;
        ui.name("events", "Opened 24")?;
        sharp_bitmap(ui, (initial.right - initial.left) * 3)
    })
}

fn sharp_bitmap(ui: &Automation, minimum_width: i32) -> Result<()> {
    let _coordinates = PhysicalCoordinates::new()?;
    let image = ui.element("bitmap-0")?;
    ui.wait(
        "enlarged bitmap resolves black and white stripes",
        || unsafe {
            let bounds = image.CurrentBoundingRectangle()?;
            let width = bounds.right - bounds.left;
            let dc = GetDC(std::ptr::null_mut());
            assert!(!dc.is_null(), "GetDC failed");
            let pixels: Vec<_> = (width / 4..width * 3 / 4)
                .map(|x| GetPixel(dc, bounds.left + x, (bounds.top + bounds.bottom) / 2))
                .collect();
            assert_ne!(ReleaseDC(std::ptr::null_mut(), dc), 0);
            assert!(!pixels.contains(&u32::MAX), "GetPixel failed");
            let min = pixels.iter().map(|pixel| pixel & 0xff).min().unwrap_or(255);
            let max = pixels.iter().map(|pixel| pixel & 0xff).max().unwrap_or(0);
            Ok((width, min, max))
        },
        |&(width, min, max)| width >= minimum_width && min < 32 && max > 223,
    )?;
    Ok(())
}

fn private_bytes(ui: &Automation) -> Result<usize> {
    unsafe {
        let mut id = 0;
        GetWindowThreadProcessId(ui.window_handle()? as HWND, &mut id);
        let process = OpenProcess(
            (PROCESS_QUERY_INFORMATION | PROCESS_VM_READ) as u32,
            false.into(),
            id,
        );
        if process.is_null() {
            return Err(windows_core::Error::from_thread().into());
        }

        let mut memory = PROCESS_MEMORY_COUNTERS_EX {
            cb: size_of::<PROCESS_MEMORY_COUNTERS_EX>() as u32,
            ..Default::default()
        };
        let result = GetProcessMemoryInfo(
            process,
            (&mut memory as *mut PROCESS_MEMORY_COUNTERS_EX).cast(),
            memory.cb,
        );
        CloseHandle(process).ok()?;
        result.ok()?;
        Ok(memory.PrivateUsage)
    }
}
