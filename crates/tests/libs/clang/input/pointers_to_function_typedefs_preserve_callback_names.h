//! namespace Callbacks
//! args -x c++ -fms-extensions

#define CALL __stdcall
             typedef void CALLBACK(int value);
             typedef void* CALL ALLOCATOR(unsigned size);
             struct TABLE { CALLBACK* callback; };
             void UseCallback(CALLBACK* callback, CALLBACK** previous, ALLOCATOR* allocator);
