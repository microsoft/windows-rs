typedef unsigned long Count;

struct Payload {
    Count count;
};

extern "C" __declspec(dllimport) Count __stdcall ReadPayload(
    const Payload* payload, Count* written);
extern "C" __declspec(dllexport) int __cdecl NativeSum(int left, int right);
