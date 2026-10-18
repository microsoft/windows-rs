#define OUTPUT __attribute__((annotate("windows_clang:sal:2:_Out_\n()\n()")))

/* [iid_is][retval][out] */
extern "C" void BufferFunction(
    int first /* [iid_is][retval][out] */,
    /* [annotation][iid_is][retval][out] */ OUTPUT void** result,
    void (*nested)(/* [iid_is][retval][out] */ void** value),
    /* prose mentioning [iid_is][retval][out] is not a MIDL attribute list */ void** prose,
    /* [out] */ OUTPUT void** ordinary);

typedef void (*Buffer)(/* [iid_is][out] */ OUTPUT void** result);
struct __declspec(uuid("22222222-2222-2222-2222-222222222222")) IBuffer {
    virtual void __stdcall Read(/* [retval][out] */ OUTPUT int* result) = 0;
};

#define MACRO_PARAMETERS /* [iid_is][retval][out] */ void** first, void** second
typedef void (*MacroBuffer)(MACRO_PARAMETERS);

typedef void (*CommentOnlyCallback)(/* [iid_is][out] */ void** value);
struct CommentCallbackStorage {
    CommentOnlyCallback named;
    void (*structural)(void** value);
};
