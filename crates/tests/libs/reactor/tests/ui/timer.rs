use super::*;

#[test]
fn timer_delivery_and_cancellation_on_component_replacement() -> Result<()> {
    process::run("timer", |ui| {
        ui.name("fired", "Fired [0]")?;
        ui.name("released", "Released []")?;
        ui.name("state", "Generation 0; retained false")?;
        let original = ui.element("replace")?;

        ui.invoke("replace")?;
        ui.name("state", "Generation 1; retained true")?;
        ui.name("released", "Released [0]")?;
        ui.name("fired", "Fired [0, 1]")?;
        unsafe {
            assert!(
                !ui.client
                    .CompareElements(&original, &ui.element("replace")?)?
                    .as_bool(),
                "keyed replacement retained the old native child"
            );
        }
        ui.name_stays("fired", "Fired [0, 1]", Duration::from_millis(250))
    })
}
