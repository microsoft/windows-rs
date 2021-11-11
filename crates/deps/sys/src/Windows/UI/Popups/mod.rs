#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IMessageDialog();
    fn IMessageDialogFactory();
    fn IPopupMenu();
    fn IUICommand();
    fn IUICommandFactory();
    fn MessageDialog();
    fn MessageDialogOptions();
    fn Placement();
    fn PopupMenu();
    fn UICommand();
    fn UICommandInvokedHandler();
    fn UICommandSeparator();
}
