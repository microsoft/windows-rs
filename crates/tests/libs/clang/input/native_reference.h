#ifndef CASE
#define CASE 0
#endif

struct __declspec(uuid("c95a92fe-21e9-41bf-8c43-4ac63a6afc41")) IBase {
    virtual long __stdcall Base(int* value) = 0;
#if CASE == 3
    virtual long __stdcall Extra() = 0;
#endif
};

#if CASE == 1
#define FOREIGN_UUID "c95a92fe-21e9-41bf-8c43-4ac63a6afc43"
#else
#define FOREIGN_UUID "c95a92fe-21e9-41bf-8c43-4ac63a6afc42"
#endif

struct __declspec(uuid(FOREIGN_UUID)) IForeign
#if CASE != 2
    : IBase
#endif
{
    virtual long __stdcall Foreign(unsigned char** value) = 0;
};

struct __declspec(uuid("c95a92fe-21e9-41bf-8c43-4ac63a6afc44")) Local : IForeign {
    virtual long __stdcall Invoke(IForeign* value) = 0;
};
