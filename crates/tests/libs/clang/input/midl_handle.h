//! flat
//! namespace Windows.Win32

// A MIDL file-scope handle placeholder. Compiling an IDL opaque/context handle
// (`typedef [handle] void *NAME`) yields a one-off `struct __MIDL___MIDL_itf_<file>_NNNN
// { int unused; }` tag plus a pointer typedef to it. The `__MIDL___MIDL_itf_*` tag is
// MIDL compiler plumbing and must never be surfaced as a type of its own:
//   * the tag struct is dropped (never emitted),
//   * the typedef collapses to an opaque `*mut void` handle, exactly like a
//     `DECLARE_HANDLE` (`struct X__ *`) tag,
//   * a by-value parameter keeps the named `NAME` alias rather than inlining the
//     anonymous `__MIDL` tag, and
//   * a pointer-to-alias parameter stays `*mut NAME`.
// Mirrors the real `UI_ANIMATION_KEYFRAME` in uianimation.h.

struct __MIDL___MIDL_itf_sample_0000_0000_0001 {
    int unused;
};
typedef struct __MIDL___MIDL_itf_sample_0000_0000_0001 *SAMPLE_KEYFRAME;

void AddAtKeyframe(SAMPLE_KEYFRAME startKeyframe);
void MakeKeyframe(SAMPLE_KEYFRAME *keyframe);
