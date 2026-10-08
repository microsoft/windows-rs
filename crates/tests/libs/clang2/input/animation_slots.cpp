#define CINTERFACE
#include <windows.h>
#include <UIAnimation.h>
#include <stddef.h>

extern "C" size_t AnimationSlot(unsigned index) {
    const size_t slots[] = {
        offsetof(IUIAnimationManager2Vtbl, CreateAnimationVariable),
        offsetof(IUIAnimationManager2Vtbl, CreateStoryboard),
        offsetof(IUIAnimationManager2Vtbl, Update),
        offsetof(IUIAnimationManager2Vtbl, ScheduleTransition),
        offsetof(IUIAnimationStoryboard2Vtbl, AddTransition),
        offsetof(IUIAnimationStoryboard2Vtbl, AddKeyframeAfterTransition),
        offsetof(IUIAnimationStoryboard2Vtbl, AddTransitionAtKeyframe),
        offsetof(IUIAnimationStoryboard2Vtbl, Schedule),
        offsetof(IUIAnimationTransitionLibrary2Vtbl, CreateAccelerateDecelerateTransition),
        offsetof(IUIAnimationTransitionLibrary2Vtbl, CreateLinearTransition),
        offsetof(IUIAnimationTransitionLibrary2Vtbl, CreateInstantaneousTransition),
        offsetof(IUIAnimationVariable2Vtbl, GetValue),
        offsetof(IUIAnimationVariable2Vtbl, GetCurve),
    };
    return slots[index];
}

extern "C" size_t AnimationKeyframeSize() {
    return sizeof(UI_ANIMATION_KEYFRAME);
}

extern "C" size_t AnimationKeyframeAlign() {
    return alignof(UI_ANIMATION_KEYFRAME);
}
