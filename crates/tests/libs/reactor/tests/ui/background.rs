use super::*;

#[test]
fn background_delivery_and_cancellation_on_component_replacement() -> Result<()> {
    process::run("background", |ui| {
        ui.name("delivered", "Delivered [0]")?;
        ui.name("released", "Released []")?;
        let original = ui.element("start")?;

        ui.invoke("start")?;
        ui.name("state", "Generation 0; retained true")?;
        ui.name("started", "Started [0]")?;
        ui.invoke("replace")?;
        ui.name("state", "Generation 1; retained true")?;
        ui.name("delivered", "Delivered [0, 1]")?;
        unsafe {
            assert!(
                !ui.client
                    .CompareElements(&original, &ui.element("start")?)?
                    .as_bool(),
                "keyed replacement retained the old native child"
            );
        }

        ui.invoke("release")?;
        ui.name("released", "Released [(0, true)]")?;
        ui.name_stays("delivered", "Delivered [0, 1]", Duration::from_millis(250))
    })
}
