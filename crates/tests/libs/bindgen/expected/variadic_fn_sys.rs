windows_link::link!("test.dll" "system" fn Fixed(count : u32) -> u32);
windows_link::link!("test.dll" "C" fn VariadicC(count : u32, ...) -> u32);
windows_link::link!("test.dll" "system" fn VariadicFunc(count : u32, ...) -> u32);
