windows_link::link!("ordinal_fixture.dll" "system" ordinal(1) fn First());
windows_link::link!("ordinal_fixture.dll" "C" ordinal(65535) fn Last(value : i32) -> i32);
