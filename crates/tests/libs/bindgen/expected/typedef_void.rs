windows_link::link!("user32.dll" "system" fn LoadMenuIndirect(template : *const MenuTemplate) -> isize);
pub type MenuTemplate = core::ffi::c_void;
pub type MenuTemplateChain = MenuTemplate;
