# windows-bindgen2

An unpublished replacement candidate for `windows-bindgen`.

The crate is developed beside the existing generator and uses `windows-metadata2` directly. Its
first layer owns the metadata database and selects deterministic WinRT value items while retaining
only typed metadata entities. A separate owned value graph now renders the complete committed
WinRT enum and struct corpus. The first flat Win32 layer selects constants and functions without
copying per-item namespaces and renders the complete function corpus plus directly typed constants.
Filters, dependency closure, native typedefs, and output layouts will be added as independent
layers with differential output tests.
