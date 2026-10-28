typedef unsigned long DWORD;
typedef int BOOL;

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

class BitmapData {
public:
    unsigned int width;
    int stride;
    void* scan;
};

class Data;
typedef Data* PData;
class Data {
public:
    unsigned int value;
};

extern "C" void UseProperty(CProperty* value, const CProperty* input);
extern "C" void UseBitmapData(BitmapData* value);
extern "C" void UseData(PData value);
