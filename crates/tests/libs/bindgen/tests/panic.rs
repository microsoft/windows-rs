#[track_caller]
fn bindgen(args: &str) -> windows_bindgen::Warnings {
    windows_bindgen::bindgen(args.split_whitespace())
}

#[test]
#[should_panic(expected = "exactly one `--out` is required")]
fn at_least_one_output() {
    bindgen("--in default");
}

#[test]
#[should_panic(expected = "exactly one `--out` is required")]
fn exactly_one_output() {
    bindgen("--in default --out a.txt b.txt");
}

#[test]
#[should_panic(expected = "at least one `--filter` required")]
fn at_least_one_filter() {
    bindgen("--in default --out out.txt ");
}

#[test]
#[should_panic(expected = "failed to open file `file_not_found.txt`")]
fn file_not_found() {
    bindgen("--etc file_not_found.txt");
}

#[test]
#[should_panic(
    expected = "failed to read file lines `../../../libs/bindgen/default/Windows.winmd`"
)]
fn not_text_file() {
    bindgen("--etc ../../../libs/bindgen/default/Windows.winmd");
}

#[test]
#[should_panic(expected = "invalid option `--invalid`")]
fn invalid_option() {
    bindgen("--invalid");
}

#[test]
#[should_panic(expected = "cannot combine `--package` and `--flat`")]
fn flat_package() {
    bindgen("--in default --flat --package");
}

#[test]
#[should_panic(expected = "`--derive` must be `<type name>=Comma,Separated,List")]
fn invalid_derive_format() {
    bindgen("--in default --out out.txt --filter POINT --derive invalid");
}

#[test]
#[should_panic(expected = "type not found: `POINTY`")]
fn invalid_derive_path() {
    bindgen("--in default --out out.txt --filter POINT --derive POINTY=PartialOrd");
}

#[test]
#[should_panic(expected = "type not included: `RECT`")]
fn excluded_derive_path() {
    bindgen("--in default --out out.txt --filter POINT --derive RECT=PartialOrd");
}

#[test]
#[should_panic(expected = "type not found: `Windows.Fondation`")]
fn invalid_filter_namespace() {
    bindgen("--in default --out out.txt --filter Windows.Fondation");
}

#[test]
#[should_panic(expected = "type not found: `AsyncStatos`")]
fn invalid_filter_name() {
    bindgen("--in default --out out.txt --filter AsyncStatos");
}

#[test]
#[should_panic(expected = "`--reference` must be `<crate>,<full/flat/skip-root>,<type name>")]
fn invalid_reference_format() {
    bindgen("--in default --out out.txt --filter POINT --reference invalid");
}

#[test]
#[should_panic(expected = "`--reference` must be `<crate>,<full/flat/skip-root>,<type name>")]
fn invalid_reference_style() {
    bindgen("--in default --out out.txt --filter POINT --reference windows,style,RECT");
}

#[test]
#[should_panic(expected = "type not found: `POINTY`")]
fn invalid_reference_type_name() {
    bindgen("--in default --out out.txt --filter POINT --reference windows,full,POINTY");
}

#[test]
#[should_panic(expected = "failed to read binary file `invalid.winmd`")]
fn invalid_input_path() {
    bindgen("--in invalid.winmd --out out.txt --filter POINT");
}

#[test]
#[should_panic(expected = "failed to find .winmd files in directory `../../libs`")]
fn input_directory_empty() {
    bindgen("--in ../../libs --out out.txt --filter POINT");
}

#[test]
#[should_panic(expected = "failed to read .winmd format `../../../libs/bindgen/default/readme.md`")]
fn invalid_input_format() {
    bindgen("--in ../../../libs/bindgen/default/readme.md --out out.txt --filter POINT");
}

#[test]
#[should_panic(expected = "type not found: `POINT`")]
fn no_default() {
    bindgen("--in ../../../libs/bindgen/default/Windows.winmd --out out.txt --filter POINT");
}

#[test]
#[should_panic(expected = "type not found: `Windows.Found`")]
fn incomplete_namespace() {
    bindgen("--in default --out out.txt --filter Windows.Found");
}

#[test]
#[should_panic(expected = "type not included: `D3D11_RESOURCE_FLAGS`")]
fn subset_namespace() {
    bindgen("--in default --out out.txt --sys --filter Windows.Win32.Graphics.Direct3D11 --derive D3D11_RESOURCE_FLAGS=Debug");
}

#[test]
#[should_panic(expected = "failed to create directory")]
fn failed_to_create_directory() {
    let test_path = format!(
        "{}\\failed_to_create_directory",
        env!("CARGO_TARGET_TMPDIR")
    );

    std::fs::write(&test_path, "test").unwrap();
    let test_path = format!("{}\\out.txt", test_path);

    bindgen(&format!("--out {test_path} --in default --filter POINT",));
}

#[test]
#[should_panic(expected = "failed to write file")]
fn failed_to_write_file() {
    let test_path = format!("{}\\failed_to_write_file", env!("CARGO_TARGET_TMPDIR"));
    std::fs::create_dir_all(&test_path).unwrap();

    bindgen(&format!("--out {test_path} --in default --filter POINT",));
}

#[test]
#[should_panic(
    expected = "skipping `Windows.Win32.System.Com.IPersistFile.Load` due to missing dependencies:\n  Windows.Win32.System.Com.STGM"
)]
fn skip_cpp_method() {
    let mut path = std::env::temp_dir();
    path.push("skip_cpp_method");

    windows_bindgen::bindgen(["--out", &path.to_string_lossy(), "--filter", "IPersistFile"])
        .unwrap();
}

#[test]
#[should_panic(
    expected = "skipping `Windows.Foundation.IMemoryBuffer.CreateReference` due to missing dependencies:\n  Windows.Foundation.IMemoryBufferReference"
)]
fn skip_method() {
    let mut path = std::env::temp_dir();
    path.push("skip_method");

    windows_bindgen::bindgen([
        "--out",
        &path.to_string_lossy(),
        "--filter",
        "IMemoryBuffer",
    ])
    .unwrap();
}

#[test]
#[should_panic(expected = "failed to format output with `rustfmt`")]
fn rustfmt_failure() {
    let mut path = std::env::temp_dir();
    path.push("rustfmt_failure");

    windows_bindgen::bindgen([
        "--out",
        &path.to_string_lossy(),
        "--rustfmt",
        "fail=true",
        "--filter",
        "POINT",
    ])
    .unwrap();
}
