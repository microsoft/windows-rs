#define EXTENT 4
enum { Count = 3 };
constexpr int extent() { return 5; }
typedef char Computed[EXTENT + 1];
using UsingArray = int[(Count + 1)];
typedef int (*PointerArray)[(EXTENT + 1)];
typedef void (__cdecl *ArrayCallback)(int value[Count + 1]);
typedef const int Values[Count];
typedef void (__cdecl *AliasCallback)(Values value, Computed bytes, int multi[Count][1 + 1]);
struct Arrays {
    char add[EXTENT + 1];
    short multiply[(Count * 2)];
    int conditional[sizeof(void*) == 8 ? 2 : 1];
    int named[Count];
    int size[sizeof(long) + 1];
    int function[extent()];
    int multi[Count][1 + 1];
    int (*pointer)[EXTENT + 1];
};
