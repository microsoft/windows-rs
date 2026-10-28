typedef long HRESULT;

struct __declspec(uuid("4b1f7c3e-0a6d-4c55-9e0b-2c8f6a1d7e02")) IProperties {
    virtual /* [propget][id] */ HRESULT __stdcall get_Name(/* [retval][out] */ int* value) = 0;
    virtual /* [propput][id] */ HRESULT __stdcall put_Name(/* [in] */ int value) = 0;
    virtual /* [id] */ HRESULT __stdcall Refresh() = 0;
};
