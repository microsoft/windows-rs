#include "sal.h"

typedef char* LPSTR;
typedef LPSTR LPTSTR;

struct Example {
    LPTSTR text;
};

extern "C" void ReadText(IN_Z const char* text);
