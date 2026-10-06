use windows_reactor::*;

#[test]
fn pane_options_accept_public_view_types() {
    let navigation: NavigationView = NavigationView::new()
        .is_back_enabled(false)
        .is_pane_visible(true)
        .pane_header("Workspace")
        .pane_footer("Account")
        .content("Page");
    let _: View = navigation.into();

    let header: View = StackPanel::new()
        .children(("Workspace", Button::new().content("Switch")))
        .into();
    let _: View = NavigationView::new()
        .is_back_enabled(true)
        .is_pane_visible(false)
        .pane_header(header)
        .into();
}
