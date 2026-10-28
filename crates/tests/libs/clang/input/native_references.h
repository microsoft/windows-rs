struct ReferenceValue {
    unsigned low;
    int high;
};
typedef ReferenceValue ReferenceAlias;
typedef unsigned (__stdcall *ReferenceCallback)(const ReferenceAlias&, unsigned&);

struct IReferences {
    virtual unsigned __stdcall Apply(const ReferenceAlias& value, unsigned& total) = 0;
};

extern "C" unsigned __stdcall ApplyReference(const ReferenceAlias& value, unsigned& total);
extern "C" unsigned __stdcall InvokeReference(ReferenceCallback callback, const ReferenceAlias& value, unsigned& total);
extern "C" unsigned __stdcall InvokeAnonymous(unsigned (__cdecl *callback)(const ReferenceAlias&, unsigned&, int), const ReferenceAlias& value, unsigned& total);
extern "C" void* __stdcall ReferenceObject();
extern "C" unsigned __stdcall InvokeObject(IReferences* object, const ReferenceAlias& value, unsigned& total);
