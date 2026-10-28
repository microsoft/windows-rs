typedef long HRESULT;

struct __declspec(uuid("4b1f7c3e-0a6d-4c55-9e0b-2c8f6a1d7e01")) IOptional {
    virtual HRESULT __stdcall Select(/* [optional][in] */ int child, /* [in] */ int flags) = 0;
    virtual HRESULT __stdcall Find(/* [in][optional] */ int* hint) = 0;
};
