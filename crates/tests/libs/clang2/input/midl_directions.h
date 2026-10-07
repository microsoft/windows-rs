#define _In_opt_ __attribute__((annotate("_In_opt_")))
#define _In_ __attribute__((annotate("_In_")))

extern "C" void SalWins(/* [annotation][out][in] */ _In_ int* value);

extern "C" void Directions(
    /* [in] */ int* input,
    /* [out] */ int* output,
    /* [in, out] */ int* update,
    /* [annotation][in] */ _In_opt_ int* optional);

struct __declspec(uuid("12345678-1234-1234-1234-123456789abc")) IDirections {
    virtual void __stdcall Apply(
        /* [in] */ int* input,
        /* [out] */ int* output,
        /* [in][out] */ int* update) = 0;
};

extern "C" void Unrelated(
    int* plain /* [in] */,
    /* Example: [in] */ int* prose,
    void (*callback)(/* [in] */ int* nested),
    /* [out] */ int* output);
