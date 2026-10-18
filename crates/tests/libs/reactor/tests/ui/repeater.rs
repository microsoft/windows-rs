use super::*;

#[test]
fn repeater_updates_preserve_realized_component_state() -> Result<()> {
    process::run("repeater", |ui| {
        ui.name("row-0", "Row 0; value 0; clicks 0")?;
        let row = ui.element("row-0")?;
        ui.invoke("row-0")?;
        ui.name("row-0", "Row 0; value 0; clicks 1")?;

        ui.invoke("rerender")?;
        ui.name("revision", "Revision 1")?;
        ui.name("row-0", "Row 0; value 0; clicks 1")?;
        ui.invoke("change")?;
        ui.name("revision", "Revision 2")?;
        ui.name("row-0", "Row 0; value 1; clicks 1")?;
        unsafe {
            assert!(
                ui.client
                    .CompareElements(&row, &ui.element("row-0")?)?
                    .as_bool(),
                "input update recreated the realized row"
            );
        }
        ui.invoke("row-0")?;
        ui.name("row-0", "Row 0; value 1; clicks 2")
    })
}
