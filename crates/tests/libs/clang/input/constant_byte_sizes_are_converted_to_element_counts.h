//! namespace ConstantByteSizes

#define READS_BYTES(c) __attribute__((annotate("_In_reads_bytes_(" #c ")")))
             extern "C" void ReadBytes(READS_BYTES(128) char* bytes);
             extern "C" void ReadWide(READS_BYTES(8) wchar_t* text);
