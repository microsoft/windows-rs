//! namespace SizedString
//! library api.dll

#define READS_OR_Z(c) __attribute__((annotate("_In_reads_or_z_(" #c ")")))
             typedef const unsigned short* PCWSTR;
             extern "C" void Read(unsigned count, READS_OR_Z(count) PCWSTR value);
