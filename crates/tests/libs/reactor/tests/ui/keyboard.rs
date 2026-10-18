use super::*;

#[test]
fn keyboard_routing_and_accelerator_callback_replacement() -> Result<()> {
    process::run("keyboard", |ui| {
        let original = ui.element("input")?;
        ui.name("state", "Handled true; generation 0")?;

        ui.press_keys("input", &[VK_CONTROL, VK_SPACE])?;
        ui.name("keys", "Preview 1; child 0")?;
        ui.press_keys("input", &[VK_SPACE])?;
        ui.name("characters", "Characters [32]")?;
        ui.press_keys("input", &[VK_CONTROL, VK_RETURN])?;
        ui.name("accelerators", "Accelerators [0]")?;

        ui.invoke("replace")?;
        ui.name("state", "Handled false; generation 1")?;
        unsafe {
            assert!(
                ui.client
                    .CompareElements(&original, &ui.element("input")?)?
                    .as_bool(),
                "callback replacement replaced the native input target"
            );
        }
        ui.press_keys("input", &[VK_CONTROL, VK_SPACE])?;
        ui.name("keys", "Preview 2; child 1")?;
        ui.press_keys("input", &[VK_SPACE])?;
        ui.name("characters", "Characters [32, 32]")?;
        ui.press_keys("input", &[VK_CONTROL, VK_RETURN])?;
        ui.name("accelerators", "Accelerators [0, 1]")?;
        ui.name_stays(
            "accelerators",
            "Accelerators [0, 1]",
            Duration::from_millis(250),
        )
    })
}
