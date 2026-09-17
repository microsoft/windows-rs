// MIDL emits per-method proxy/stub marshaling thunks into the generated header
// (the `*_p.c` proxy file). They follow the `{Interface}_{Method}_Proxy` /
// `_Stub` naming convention with a generated explicit `This` first parameter, and
// must be dropped: they are RPC plumbing, not public API. A real flat C function
// that merely happens to end in `_Stub` (no `This` parameter) must be kept.
struct IFoo;

int IFoo_GetValue_Proxy(struct IFoo* This, int* value);
void IFoo_GetValue_Stub(struct IFoo* This, int* value);

// Not a MIDL thunk: kept despite the suffix, because its first parameter is not
// the generated `This`.
int Register_Stub(int id);
