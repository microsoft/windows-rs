//! namespace Redeclared
//! library api.dll
#define DECLARE(name, parameter) extern "C" int name(int parameter)
DECLARE(Shared, first);
DECLARE(Shared, second);
