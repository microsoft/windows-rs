use crate::controls::*;
use windows_reactor::*;

pub fn list_view_page(_: &(), cx: &mut RenderCx) -> Element {
    let (selected_contact, set_selected_contact) = cx.use_state(1_i32);
    let (selected_playlist, set_selected_playlist) = cx.use_state(0_i32);

    let inbox_items: Vec<String> = [
        "Quarterly planning notes",
        "Design review follow-up",
        "Flight confirmation",
        "Weekly status summary",
        "Welcome to the team",
        "Shared gallery feedback",
    ]
    .into_iter()
    .map(String::from)
    .collect();

    let contacts: Vec<String> = ["Avery", "Diego", "Harper", "Mina", "Noah"]
        .into_iter()
        .map(String::from)
        .collect();
    let contact_label = contacts
        .get(selected_contact.max(0) as usize)
        .map_or("(none)", String::as_str);

    let playlists: Vec<String> = ["Morning Focus", "Deep Work", "Evening Reset", "Weekend Mix"]
        .into_iter()
        .map(String::from)
        .collect();
    let playlist_label = playlists
        .get(selected_playlist.max(0) as usize)
        .map_or("(none)", String::as_str);

    page_content(
        "ListView",
        "Displays rich, scrollable collections that can react to selection changes.",
        vec![
            sample_card(
                "Basic ListView",
                list_view(inbox_items, |item, idx| {
                    vstack((
                        text_block(item.clone()).bold(),
                        text_block(format!("Message #{} • Updated just now", idx + 1)).opacity(0.6),
                    ))
                    .spacing(2.0)
                    .padding(Thickness::uniform(8.0))
                })
                .with_key_selector(|item| item.clone())
                .height(220.0),
                r#"list_view(items, |item, idx| {
    vstack((text_block(item.clone()).bold(), ...)).padding(Thickness::uniform(8.0))
})"#,
            ),
            sample_card(
                "Selection Display",
                vstack((
                    list_view(contacts.clone(), |item, _idx| {
                        text_block(item.clone()).padding(Thickness::uniform(10.0))
                    })
                    .with_key_selector(|item| item.clone())
                    .selected_index(selected_contact)
                    .on_selection_changed({
                        let set_selected_contact = set_selected_contact;
                        move |index| set_selected_contact.call(index)
                    })
                    .height(180.0),
                    text_block(format!("Selected contact: {contact_label}")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"list_view(contacts, |item, _| text_block(item.clone()))
    .selected_index(selected_contact)
    .on_selection_changed(...)"#,
            ),
            sample_card(
                "Playlist Browser",
                vstack((
                    list_view(playlists.clone(), |item, idx| {
                        vstack((
                            text_block(item.clone()).bold(),
                            text_block(format!("{} tracks ready to play", 12 + idx * 5))
                                .opacity(0.6),
                        ))
                        .spacing(2.0)
                        .padding(Thickness::uniform(10.0))
                    })
                    .with_key_selector(|item| item.clone())
                    .selected_index(selected_playlist)
                    .on_selection_changed({
                        let set_selected_playlist = set_selected_playlist;
                        move |index| set_selected_playlist.call(index)
                    })
                    .height(200.0),
                    text_block(format!("Now browsing: {playlist_label}")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"list_view(playlists, |item, idx| {
    vstack((text_block(item.clone()).bold(), ...))
})
.selected_index(selected_playlist)
.on_selection_changed(...)"#,
            ),
        ],
    )
}
