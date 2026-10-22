#pragma once

typedef char16_t Unit16;
typedef char32_t Unit32;
enum class Maximum16 : char16_t { Value = 0xffff };
enum class Maximum32 : char32_t { Value = 0xffffffffu };
typedef void (*Char16Callback)(char16_t);
typedef void (*Char32Callback)(char32_t);
typedef void (*Unsigned16Callback)(unsigned short);
typedef void (*Unsigned32Callback)(unsigned int);

struct CharacterCallbacks {
    void (*character16)(char16_t);
    void (*character32)(char32_t);
    void (*integer16)(unsigned short);
    void (*integer32)(unsigned int);
};
struct Characters {
    char16_t first;
    char32_t second;
    const char16_t* text;
    char32_t buffer[2];
    Unit16 named16;
    Unit32 named32;
};
struct Compare16 { char16_t unit; };
struct Compare32 { char32_t unit; };
