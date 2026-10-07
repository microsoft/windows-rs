struct __declspec(uuid("f0f0f001-1234-5678-1234-567812345678")) Properties {
    virtual /* [propget] */ int __stdcall get_Value(/* [out] */ int* value) = 0;
    virtual /* [propput] */ int __stdcall put_Value(/* [in] */ int value) = 0;
    virtual int __stdcall get_Plain(/* [propget][out] */ int* value) = 0;
};
