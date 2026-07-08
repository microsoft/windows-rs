//! namespace Test
//! library user32.dll

// A SAL-annotated opaque handle (the `HSTRING` shape). Unlike the bare DECLARE_HANDLE
// idiom, the dummy `X__` tag struct is declared separately and the pointer typedef
// carries a SAL `__attribute__((annotate(...)))` wrapper (what `__RPC_unique_pointer`
// expands to under the SAL shim). The attribute must not defeat the handle collapse:
// `RoHandle` must still become an opaque `*mut void`, and the `RoHandle__` dummy struct
// must not be emitted (otherwise the typedef would dangle on an unemitted tag).
typedef struct RoHandle__ {
    int unused;
} RoHandle__;

typedef __attribute__((annotate("_Maybenull_"))) RoHandle__* RoHandle;

RoHandle MakeRoHandle(void);
