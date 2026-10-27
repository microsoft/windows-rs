typedef void Nothing;
typedef Nothing Again;
typedef Again* Address;
typedef const Again* ConstAddress;
typedef Nothing (__stdcall *Callback)(Address value);
typedef Again (__stdcall Function)(ConstAddress value);

struct Holder {
    Again* direct_value;
    const Nothing* qualified_value;
    Address mutable_value;
    ConstAddress constant_value;
    Callback callback;
    Function* function;
};

#if defined(_M_X64)
typedef Holder TargetValue;
#else
typedef Nothing TargetValue;
#endif

struct __declspec(uuid("4A79E274-0DB1-4CE0-A852-432B6D179303")) IConsumer {
    virtual Nothing __stdcall Clear() = 0;
    virtual Again __stdcall Set(ConstAddress value) = 0;
};

extern "C" Nothing __stdcall Clear(Again);
extern "C" Again __stdcall Set(ConstAddress value);
extern "C" Address __stdcall Get();
