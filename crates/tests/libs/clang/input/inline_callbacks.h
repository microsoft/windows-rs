#pragma once

typedef unsigned int Count;
struct Payload { int value; };
typedef int (__cdecl *Named)(int);

struct InlineCallbacks {
    int (__cdecl *named)(int);
    Count (__cdecl *count)(const Payload*, Count);
    void (__stdcall *system)(Payload*);
    void (__cdecl *empty)();
    int (__cdecl **pointer)(short);
    void (__cdecl *array[2])(char);
    int (__cdecl *(__cdecl *factory)(void (__stdcall *)(Payload*)))(short);
    void (__cdecl *arrays)(const Count values[3], int matrix[2][3]);
    void (__cdecl *function)(Count callback(Count));
    const int** (__cdecl *mixed)(int* const*);
    InlineCallbacks* (__cdecl *recursive)(InlineCallbacks*);
};
