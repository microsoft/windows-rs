#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IMessageDialog(i32);
pub struct IMessageDialogFactory(i32);
pub struct IPopupMenu(i32);
pub struct IUICommand(i32);
pub struct IUICommandFactory(i32);
pub struct MessageDialog(i32);
pub struct MessageDialogOptions(i32);
pub struct Placement(i32);
pub struct PopupMenu(i32);
pub struct UICommand(i32);
pub struct UICommandInvokedHandler(i32);
pub struct UICommandSeparator(i32);
