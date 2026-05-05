// Variadic function declarations.  A variadic C++ function (no extern "C"
// linkage) is emitted with default ABI.  An extern "C" variadic function
// is emitted with "C" ABI.  Both must carry the trailing `...` parameter.
int VarArgFn(unsigned int count, ...);

extern "C"
{
    int VarArgCFn(unsigned int count, ...);
}
