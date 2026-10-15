#include "aggregate_constants.h"
int RuntimeValue();
const GUID SIDE_EFFECT = {(unsigned long)RuntimeValue(), 0, 0, {}};
extern const GUID UNDEFINED;
const wchar_t INVALID_UTF16[] = {0xd800, 0};
const char UNTERMINATED[] = {'a', 'b'};
const char NON_ASCII[] = "\xff";
struct WrongGuid {
    unsigned short Data1;
    unsigned short Data2;
    unsigned short Data3;
    unsigned char Data4[8];
};
const WrongGuid WRONG_SHAPE = {};
