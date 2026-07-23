use crate::controls::*;
use windows_reactor::*;

pub fn command_bar_page(_: &(), cx: &mut RenderCx) -> Element {
    let (status, set_status) = cx.use_state(String::from("Choose a command"));

    page_content(
        "CommandBar",
        "A toolbar for app commands and actions.",
        vec![
            sample_card(
                "Interactive CommandBar",
                vstack((
                    command_bar(vec![
                        CommandBarCommandDef::Button {
                            label: "Add".into(),
                            icon: Some(Symbol::Add.into()),
                        },
                        CommandBarCommandDef::Button {
                            label: "Edit".into(),
                            icon: Some(Symbol::Edit.into()),
                        },
                        CommandBarCommandDef::Button {
                            label: "Delete".into(),
                            icon: Some(Symbol::Delete.into()),
                        },
                    ])
                    .on_click({
                        let set_status = set_status;
                        move |label| set_status.call(format!("Last command: {label}"))
                    }),
                    text_block(status).opacity(0.6),
                ))
                .spacing(8.0),
                r#"command_bar(commands).on_click(|label| set_status.call(format!("Last command: {label}")))"#,
            ),
            sample_card(
                "Basic CommandBar",
                command_bar(vec![
                    CommandBarCommandDef::Button {
                        label: "Add".into(),
                        icon: Some(Symbol::Add.into()),
                    },
                    CommandBarCommandDef::Button {
                        label: "Edit".into(),
                        icon: Some(Symbol::Edit.into()),
                    },
                    CommandBarCommandDef::Button {
                        label: "Delete".into(),
                        icon: Some(Symbol::Delete.into()),
                    },
                ]),
                r#"command_bar(vec![CommandBarCommandDef::Button { label, icon }])"#,
            ),
        ],
    )
}
