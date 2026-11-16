use super::*;

#[test]
fn selection_contracts_cover_navigation_list_box_and_selector_bar() {
    for (owner, item, relations, payload) in [
        (
            ObjectType::NavigationView,
            ObjectType::NavigationViewItem,
            &[RelationId::MenuItems, RelationId::FooterMenuItems][..],
            PropertyId::Tag,
        ),
        (
            ObjectType::ListBox,
            ObjectType::ListBoxItem,
            &[RelationId::Items][..],
            PropertyId::Tag,
        ),
        (
            ObjectType::SelectorBar,
            ObjectType::SelectorBarItem,
            &[RelationId::Items][..],
            PropertyId::Text,
        ),
    ] {
        let contract = selection_contract(owner).unwrap();
        assert_eq!(contract.item, item);
        assert_eq!(contract.relations, relations);
        assert_eq!(contract.selected_property, PropertyId::IsSelected);
        assert_eq!(contract.event, EventId::SelectionChanged);
        assert_eq!(contract.payload_property, payload);
    }
}
