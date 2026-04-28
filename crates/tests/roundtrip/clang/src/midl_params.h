// Test MIDL-style block-comment parameter annotations.
//
// MIDL generates a block comment immediately before each parameter type/name to
// record the parameter's IDL attributes, for example:
//
//   virtual HRESULT __stdcall GetValue(/* [retval][out] */ int *result) = 0;
//
// These comments are extracted by tokenising the ParmDecl cursor extent and
// looking for CXToken_Comment tokens that contain known MIDL attribute names.

struct __declspec(uuid("aabbccdd-1234-5678-9abc-def012345678"))
IFoo {
    // [in] on a value type: direction matches default (In), no extra attribute.
    virtual int __stdcall Method1(
        /* [in] */ int count) = 0;

    // [out] on a mutable pointer: direction matches default (Out), no extra attribute.
    virtual int __stdcall Method2(
        /* [out] */ int *result) = 0;

    // [retval][out] on a mutable pointer: Out direction + #[retval] attribute.
    virtual int __stdcall Method3(
        /* [retval][out] */ int *value) = 0;

    // [in] on a mutable pointer: overrides default Out, so #[in] is emitted.
    virtual int __stdcall Method4(
        /* [in] */ int *data) = 0;

    // [in][optional]: optional flag is emitted alongside default-In direction.
    virtual int __stdcall Method5(
        /* [in][optional] */ const char *name) = 0;
};
