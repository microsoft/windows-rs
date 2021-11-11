#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AddContactResult();
    fn ContactPickerUI();
    fn ContactRemovedEventArgs();
    fn IContactPickerUI();
    fn IContactPickerUI2();
    fn IContactRemovedEventArgs();
}
