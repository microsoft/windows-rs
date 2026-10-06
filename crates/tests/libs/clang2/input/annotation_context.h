#ifdef REVERSE
extern "C" void Fill(int renamed, int* buffer);
#endif
extern "C" void Fill(
    int count,
    int* __attribute__((annotate("_Out_writes_(count)"))) buffer);
#ifndef REVERSE
extern "C" void Fill(int renamed, int* buffer);
#endif
