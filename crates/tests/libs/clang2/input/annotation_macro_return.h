#define DECLARE_RESULT(type) extern "C" __declspec(dllimport) type __stdcall
#define DECLARE_METHOD(type) virtual type __stdcall

#ifdef REDECLARE_FIRST
DECLARE_RESULT(int) Query(int earlier);
#endif
_Success_(return >= count)
DECLARE_RESULT(int) Query(int count);
#ifndef REDECLARE_FIRST
DECLARE_RESULT(int) Query(int later);
#endif

_Success_(return >= amount)
DECLARE_RESULT(int) Mixed(int amount) _Ret_range_(0, 100);

struct __declspec(uuid("d8e52d74-e90f-4f9f-9817-e19352eaa808")) IQuery {
    _Success_(return >= limit)
    DECLARE_METHOD(int) Get(int limit) = 0;
};
