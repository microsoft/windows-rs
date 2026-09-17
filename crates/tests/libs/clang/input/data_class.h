typedef unsigned long DWORD;
typedef int BOOL;

class CProperty
{
public:
    DWORD id;
    union
    {
        void* pointer;
        DWORD value;
    };
    BOOL persistent;
};

typedef CProperty* LPPROP;

class IPropertyList
{
public:
    virtual int Add(CProperty& value) = 0;
};

class BitmapData
{
public:
    unsigned int width;
    int stride;
    void* scan;
};

extern "C" void UseProperty(CProperty* value, const CProperty* input);
extern "C" void UseBitmapData(BitmapData* value);
