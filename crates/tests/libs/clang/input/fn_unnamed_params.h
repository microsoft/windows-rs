//! library user32.dll
// C permits a function prototype to declare a parameter by type only, with no
// name (`void Func(int)`). Metadata requires every parameter to be named, so an
// unnamed parameter is synthesized as `param{index}` (0-based), matching the
// shipped Win32 metadata convention.

extern "C" {
    // Both parameters are unnamed -> param0, param1.
    int __stdcall TwoUnnamed(int, long);

    // A mix of named and unnamed -> named stays, the gap fills with its index.
    int __stdcall MixedNames(int first, long, int third);
}
