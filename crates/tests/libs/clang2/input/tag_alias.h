typedef struct Record Record;
struct Record { int value; };
typedef struct IFoo IFoo;
struct __declspec(uuid("00000001-0000-0000-c000-000000000046")) IFoo {
    virtual void __stdcall Call(Record* value) = 0;
};
