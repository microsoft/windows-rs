struct Type {};

struct ITypes {
    // Void
    virtual void __stdcall VoidPtrMut(void* a) = 0;           // PtrMut(Void, 1)
    virtual void __stdcall VoidPtrMut2(void** a) = 0;          // PtrMut(Void, 2)
    virtual void __stdcall VoidPtrConst(void const* a) = 0;    // PtrConst(Void, 1)

    // Bool
    virtual void __stdcall BoolValue(bool a) = 0;              // Bool
    virtual void __stdcall BoolRefMut(bool& a) = 0;            // RefMut(Bool)
    virtual void __stdcall BoolRefConst(bool const& a) = 0;    // RefConst(Bool)
    virtual void __stdcall BoolPtrMut(bool* a) = 0;            // PtrMut(Bool, 1)
    virtual void __stdcall BoolPtrMut2(bool** a) = 0;          // PtrMut(Bool, 2)
    virtual void __stdcall BoolPtrConst(bool const* a) = 0;    // PtrConst(Bool, 1)
    virtual void __stdcall BoolArrayFixed(bool a[5]) = 0;      // ArrayFixed(Bool, 5)

    // I8 (signed char)
    virtual void __stdcall I8Value(signed char a) = 0;
    virtual void __stdcall I8RefMut(signed char& a) = 0;
    virtual void __stdcall I8RefConst(signed char const& a) = 0;
    virtual void __stdcall I8PtrMut(signed char* a) = 0;
    virtual void __stdcall I8PtrMut2(signed char** a) = 0;
    virtual void __stdcall I8PtrConst(signed char const* a) = 0;
    virtual void __stdcall I8ArrayFixed(signed char a[5]) = 0;

    // U8 (unsigned char)
    virtual void __stdcall U8Value(unsigned char a) = 0;
    virtual void __stdcall U8RefMut(unsigned char& a) = 0;
    virtual void __stdcall U8RefConst(unsigned char const& a) = 0;
    virtual void __stdcall U8PtrMut(unsigned char* a) = 0;
    virtual void __stdcall U8PtrMut2(unsigned char** a) = 0;
    virtual void __stdcall U8PtrConst(unsigned char const* a) = 0;
    virtual void __stdcall U8ArrayFixed(unsigned char a[5]) = 0;

    // I16 (short)
    virtual void __stdcall I16Value(short a) = 0;
    virtual void __stdcall I16RefMut(short& a) = 0;
    virtual void __stdcall I16RefConst(short const& a) = 0;
    virtual void __stdcall I16PtrMut(short* a) = 0;
    virtual void __stdcall I16PtrMut2(short** a) = 0;
    virtual void __stdcall I16PtrConst(short const* a) = 0;
    virtual void __stdcall I16ArrayFixed(short a[5]) = 0;

    // U16 (unsigned short)
    virtual void __stdcall U16Value(unsigned short a) = 0;
    virtual void __stdcall U16RefMut(unsigned short& a) = 0;
    virtual void __stdcall U16RefConst(unsigned short const& a) = 0;
    virtual void __stdcall U16PtrMut(unsigned short* a) = 0;
    virtual void __stdcall U16PtrMut2(unsigned short** a) = 0;
    virtual void __stdcall U16PtrConst(unsigned short const* a) = 0;
    virtual void __stdcall U16ArrayFixed(unsigned short a[5]) = 0;

    // I32 (int)
    virtual void __stdcall I32Value(int a) = 0;
    virtual void __stdcall I32RefMut(int& a) = 0;
    virtual void __stdcall I32RefConst(int const& a) = 0;
    virtual void __stdcall I32PtrMut(int* a) = 0;
    virtual void __stdcall I32PtrMut2(int** a) = 0;
    virtual void __stdcall I32PtrConst(int const* a) = 0;
    virtual void __stdcall I32ArrayFixed(int a[5]) = 0;

    // U32 (unsigned int)
    virtual void __stdcall U32Value(unsigned int a) = 0;
    virtual void __stdcall U32RefMut(unsigned int& a) = 0;
    virtual void __stdcall U32RefConst(unsigned int const& a) = 0;
    virtual void __stdcall U32PtrMut(unsigned int* a) = 0;
    virtual void __stdcall U32PtrMut2(unsigned int** a) = 0;
    virtual void __stdcall U32PtrConst(unsigned int const* a) = 0;
    virtual void __stdcall U32ArrayFixed(unsigned int a[5]) = 0;

    // I64 (long long)
    virtual void __stdcall I64Value(long long a) = 0;
    virtual void __stdcall I64RefMut(long long& a) = 0;
    virtual void __stdcall I64RefConst(long long const& a) = 0;
    virtual void __stdcall I64PtrMut(long long* a) = 0;
    virtual void __stdcall I64PtrMut2(long long** a) = 0;
    virtual void __stdcall I64PtrConst(long long const* a) = 0;
    virtual void __stdcall I64ArrayFixed(long long a[5]) = 0;

    // U64 (unsigned long long)
    virtual void __stdcall U64Value(unsigned long long a) = 0;
    virtual void __stdcall U64RefMut(unsigned long long& a) = 0;
    virtual void __stdcall U64RefConst(unsigned long long const& a) = 0;
    virtual void __stdcall U64PtrMut(unsigned long long* a) = 0;
    virtual void __stdcall U64PtrMut2(unsigned long long** a) = 0;
    virtual void __stdcall U64PtrConst(unsigned long long const* a) = 0;
    virtual void __stdcall U64ArrayFixed(unsigned long long a[5]) = 0;

    // F32 (float)
    virtual void __stdcall F32Value(float a) = 0;
    virtual void __stdcall F32RefMut(float& a) = 0;
    virtual void __stdcall F32RefConst(float const& a) = 0;
    virtual void __stdcall F32PtrMut(float* a) = 0;
    virtual void __stdcall F32PtrMut2(float** a) = 0;
    virtual void __stdcall F32PtrConst(float const* a) = 0;
    virtual void __stdcall F32ArrayFixed(float a[5]) = 0;

    // F64 (double)
    virtual void __stdcall F64Value(double a) = 0;
    virtual void __stdcall F64RefMut(double& a) = 0;
    virtual void __stdcall F64RefConst(double const& a) = 0;
    virtual void __stdcall F64PtrMut(double* a) = 0;
    virtual void __stdcall F64PtrMut2(double** a) = 0;
    virtual void __stdcall F64PtrConst(double const* a) = 0;
    virtual void __stdcall F64ArrayFixed(double a[5]) = 0;

    // U16 (wchar_t - an alias for u16 in Windows metadata)
    virtual void __stdcall WCharValue(wchar_t a) = 0;
    virtual void __stdcall WCharRefMut(wchar_t& a) = 0;
    virtual void __stdcall WCharRefConst(wchar_t const& a) = 0;
    virtual void __stdcall WCharPtrMut(wchar_t* a) = 0;
    virtual void __stdcall WCharPtrMut2(wchar_t** a) = 0;
    virtual void __stdcall WCharPtrConst(wchar_t const* a) = 0;
    virtual void __stdcall WCharArrayFixed(wchar_t a[5]) = 0;

    // ValueName (user-defined struct)
    virtual void __stdcall TypeValue(Type a) = 0;
    virtual void __stdcall TypeRefMut(Type& a) = 0;
    virtual void __stdcall TypeRefConst(Type const& a) = 0;
    virtual void __stdcall TypePtrMut(Type* a) = 0;
    virtual void __stdcall TypePtrMut2(Type** a) = 0;
    virtual void __stdcall TypePtrConst(Type const* a) = 0;
    virtual void __stdcall TypeArrayFixed(Type a[5]) = 0;
};
