#include <sal.h>

_Check_return_
extern "C" int Status();
extern "C" void Plain();

#if 0
/* interface Wrong */
/* [object][local] */
struct Wrong;
#endif

/* interface Root */
/* [object][local] */
typedef struct Root Root;
extern int Companion;
extern "C++" {
struct __declspec(uuid("00112233-4455-6677-8899-aabbccddeeff")) Root {
    virtual void __stdcall Method(_In_ int* value) = 0;
};
}

/* interface Other */
/* [object] */
struct __declspec(uuid("11223344-5566-7788-99aa-bbccddeeff00")) Other {
    virtual void __stdcall Method() = 0;
};
