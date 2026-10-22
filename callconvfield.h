// bindgen-flags: --raw-line "#![cfg(not(test))]" -- --target=i686-pc-win32
// bindgen-unstable
//
// We can only check that this builds, but not that it actually passes, because
// we have no CI on the target platform.

struct JNINativeInterface_ {
  int (__stdcall *GetVersion)(void *env);
  unsigned long long __hack; // A hack so the field alignment is the same than
                             // for 64-bit, where we run CI.
};

__stdcall void bar();
