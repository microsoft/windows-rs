#pragma once
#define PUBLIC_TYPE __attribute__((type_visibility("default")))
struct PUBLIC_TYPE Visible { int value; };
union PUBLIC_TYPE VisibleUnion { int value; char bytes[4]; };
enum PUBLIC_TYPE VisibleEnum : int { VisibleValue = 7 };
struct Forward;
struct PUBLIC_TYPE Forward { int value; };
typedef struct _GUID {
    unsigned long Data1;
    unsigned short Data2;
    unsigned short Data3;
    unsigned char Data4[8];
} Guid;
struct Container {
    Visible value;
    VisibleUnion choice;
    VisibleEnum kind;
    Forward* next;
    Guid id;
};
