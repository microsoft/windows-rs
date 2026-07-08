// When an IDL method parameter is declared without a name, the MIDL compiler
// synthesises one of the form `__MIDL__<Interface>NNNN` (e.g. `__MIDL__IHolder0000`)
// in the generated header. These compiler-internal spellings must not surface as
// parameter names: `parse_params` (clang/annotation.rs) treats a
// `is_midl_synthetic_param_name` spelling the same as an empty name and falls back to
// its positional `param{idx}` naming. Real parameter names are left untouched.

typedef int HRESULT;

struct IHolder {
    // Unnamed IDL params -> MIDL synthesises `__MIDL__IHolder0000` / `...0001`;
    // both must be replaced by the positional fallback (`param0`, `param1`).
    virtual HRESULT TrackResource(int __MIDL__IHolder0000) = 0;
    virtual HRESULT UntrackResource(int __MIDL__IHolder0001, int __MIDL__IHolder0002) = 0;

    // A real parameter name is preserved verbatim.
    virtual HRESULT FreeResource(int resource) = 0;
};
