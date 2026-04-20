// Structs, enums, typedefs, callbacks and function declarations that appear
// inside extern "C" { } blocks must be parsed the same way as top-level
// declarations.  MIDL-generated headers wrap virtually everything in such a
// block.

extern "C"
{
    struct ExternStruct {
        int x;
        int y;
    };

    enum ExternEnum {
        ExternEnumA = 0,
        ExternEnumB = 1,
    };

    typedef unsigned int ExternTypedef;

    typedef void (*ExternCallback)(int a);

    void ExternFn(unsigned int a, int b);
}
