// Tests UUID extraction via __attribute__((annotate("..."))) — the
// CXCursor_AnnotateAttr path in extract_uuid().  The existing interface.h and
// midl_interface.h tests only cover the __declspec(uuid(...)) /
// CXCursor_UnexposedAttr path.

struct __attribute__((annotate("11111111-1111-1111-1111-111111111111")))
IAnnotated {
    virtual int __stdcall Method(int x) = 0;
};
