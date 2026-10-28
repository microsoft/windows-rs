#pragma once
typedef struct _LsaText {
    unsigned short Length;
    unsigned short MaximumLength;
    wchar_t* Buffer;
} LsaText;
