use super::*;

#[test]
fn webview_messages_and_subscription_retirement() -> Result<()> {
    process::run("webview", |ui| {
        ui.name("received", "Browser 0: ready; messages 1")?;
        ui.name("released", "Released []")?;
        ui.invoke("send")?;
        ui.name("received", "Browser 0: echo:1:ping; messages 2")?;
        ui.invoke("send")?;
        ui.name("received", "Browser 0: echo:2:ping; messages 3")?;

        ui.invoke("remove")?;
        ui.gone("send")?;
        ui.name("released", "Released [0]")?;
        ui.invoke("attach")?;
        ui.name("received", "Browser 1: ready; messages 4")?;
        ui.invoke("send")?;
        ui.name("received", "Browser 1: echo:1:ping; messages 5")?;
        ui.name("released", "Released [0]")?;
        ui.invoke("remove")?;
        ui.gone("send")?;
        ui.name("released", "Released [0, 1]")?;
        ui.name_stays(
            "received",
            "Browser 1: echo:1:ping; messages 5",
            Duration::from_millis(250),
        )
    })
}
