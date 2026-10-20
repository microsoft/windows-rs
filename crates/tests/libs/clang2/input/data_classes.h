// Regression for windows-rs#4932 and the legacy data_classes fixture.
typedef unsigned long DWORD;
typedef int BOOL;
class Property {
public:
    unsigned long id;
    void* value;
};
class CProperty {
public:
    DWORD id;
    union {
        void* pointer;
        DWORD value;
    };
    BOOL persistent;
};
typedef CProperty* LPPROP;
class IPropertyList {
public:
    virtual int __stdcall Add(CProperty& value) = 0;
};
class BitmapData {
public:
    unsigned int width;
    int stride;
    void* scan;
};
extern "C" void UseProperty(CProperty* value, const CProperty* input);
extern "C" void UseBitmapData(BitmapData* value);
