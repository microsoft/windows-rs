//! namespace UnresolvedSal

#define WRITES(c) __attribute__((annotate("_Out_writes_(" #c ")")))
             extern "C" void Write(unsigned targetIdCount,                  WRITES(targetCount) int* values);
