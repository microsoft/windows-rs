#define STDMETHOD_(type, method) virtual type __stdcall method
#define DrawText DrawTextW
#define GetMessage GetMessageA
#define Inner InnerW
#define Chained Inner
#define Paste(name) name##Pasted
#define Produced() ProducedName

struct WrittenFields {
    int GetMessage;
    int Chained;
    int Paste(Field);
};

struct __declspec(uuid("0D5D1B83-3E4F-4B44-8A54-2C0F7F5C6E21")) IWrittenNames {
    virtual int __stdcall DrawText(int Chained) = 0;
    STDMETHOD_(int, GetMessage)(int Inner) = 0;
    virtual int __stdcall Paste(Draw)() = 0;
    virtual int __stdcall Produced()() = 0;
    friend struct WrittenFields;
};

extern "C" int __stdcall DrawText(int Chained);

#undef Inner
