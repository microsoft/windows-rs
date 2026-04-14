/*
**  Copyright (c) Microsoft Corporation. All rights reserved.
**  Interaction Context API
*/

#ifndef INTERACTION_CONTEXT_H
#define INTERACTION_CONTEXT_H

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#include <windows.h>
#include <float.h>

#if defined(__cplusplus)
extern "C" {
#endif

#if (NTDDI_VERSION >= NTDDI_WIN8)

typedef enum INTERACTION_ID
{
    INTERACTION_ID_NONE           = 0x00000000,
    INTERACTION_ID_MANIPULATION   = 0x00000001,
    INTERACTION_ID_TAP            = 0x00000002,
    INTERACTION_ID_SECONDARY_TAP  = 0x00000003,
    INTERACTION_ID_HOLD           = 0x00000004,
    INTERACTION_ID_DRAG           = 0x00000005,
    INTERACTION_ID_CROSS_SLIDE    = 0x00000006,
    INTERACTION_ID_MAX            = 0xffffffff
} INTERACTION_ID;

typedef enum INTERACTION_FLAGS
{
    INTERACTION_FLAG_NONE    = 0x00000000,
    INTERACTION_FLAG_BEGIN   = 0x00000001,
    INTERACTION_FLAG_END     = 0x00000002,
    INTERACTION_FLAG_CANCEL  = 0x00000004,
    INTERACTION_FLAG_INERTIA = 0x00000008,
    INTERACTION_FLAG_MAX     = 0xffffffff
} INTERACTION_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS(INTERACTION_FLAGS);

typedef enum INTERACTION_CONFIGURATION_FLAGS
{
    INTERACTION_CONFIGURATION_FLAG_NONE                                     = 0x00000000,

    INTERACTION_CONFIGURATION_FLAG_MANIPULATION                             = 0x00000001,
    INTERACTION_CONFIGURATION_FLAG_MANIPULATION_TRANSLATION_X               = 0x00000002,
    INTERACTION_CONFIGURATION_FLAG_MANIPULATION_TRANSLATION_Y               = 0x00000004,
    INTERACTION_CONFIGURATION_FLAG_MANIPULATION_ROTATION                    = 0x00000008,
    INTERACTION_CONFIGURATION_FLAG_MANIPULATION_SCALING                     = 0x00000010,
    INTERACTION_CONFIGURATION_FLAG_MANIPULATION_TRANSLATION_INERTIA         = 0x00000020,
    INTERACTION_CONFIGURATION_FLAG_MANIPULATION_ROTATION_INERTIA            = 0x00000040,
    INTERACTION_CONFIGURATION_FLAG_MANIPULATION_SCALING_INERTIA             = 0x00000080,
    INTERACTION_CONFIGURATION_FLAG_MANIPULATION_RAILS_X                     = 0x00000100,
    INTERACTION_CONFIGURATION_FLAG_MANIPULATION_RAILS_Y                     = 0x00000200,
    INTERACTION_CONFIGURATION_FLAG_MANIPULATION_EXACT                       = 0x00000400,
    INTERACTION_CONFIGURATION_FLAG_MANIPULATION_MULTIPLE_FINGER_PANNING     = 0x00000800,

    INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE                              = 0x00000001,
    INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE_HORIZONTAL                   = 0x00000002,
    INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE_SELECT                       = 0x00000004,
    INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE_SPEED_BUMP                   = 0x00000008,
    INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE_REARRANGE                    = 0x00000010,
    INTERACTION_CONFIGURATION_FLAG_CROSS_SLIDE_EXACT                        = 0x00000020,

    INTERACTION_CONFIGURATION_FLAG_TAP                                      = 0x00000001,
    INTERACTION_CONFIGURATION_FLAG_TAP_DOUBLE                               = 0x00000002,
#if (NTDDI_VERSION >= NTDDI_WIN10_VB)
    INTERACTION_CONFIGURATION_FLAG_TAP_MULTIPLE_FINGER                      = 0x00000004,
#endif

    INTERACTION_CONFIGURATION_FLAG_SECONDARY_TAP                            = 0x00000001,

    INTERACTION_CONFIGURATION_FLAG_HOLD                                     = 0x00000001,
    INTERACTION_CONFIGURATION_FLAG_HOLD_MOUSE                               = 0x00000002,
#if (NTDDI_VERSION >= NTDDI_WIN10_VB)
    INTERACTION_CONFIGURATION_FLAG_HOLD_MULTIPLE_FINGER                     = 0x00000004,
#endif

    INTERACTION_CONFIGURATION_FLAG_DRAG                                     = 0x00000001,

    INTERACTION_CONFIGURATION_FLAG_MAX                                      = 0xffffffff
} INTERACTION_CONFIGURATION_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS(INTERACTION_CONFIGURATION_FLAGS);

typedef enum INERTIA_PARAMETER
{
    INERTIA_PARAMETER_TRANSLATION_DECELERATION = 0x00000001,
    INERTIA_PARAMETER_TRANSLATION_DISPLACEMENT = 0x00000002,

    INERTIA_PARAMETER_ROTATION_DECELERATION    = 0x00000003,
    INERTIA_PARAMETER_ROTATION_ANGLE           = 0x00000004,

    INERTIA_PARAMETER_EXPANSION_DECELERATION   = 0x00000005,
    INERTIA_PARAMETER_EXPANSION_EXPANSION      = 0x00000006,

    INERTIA_PARAMETER_MAX                      = 0xffffffff
} INERTIA_PARAMETER;

typedef enum INTERACTION_STATE
{
    INTERACTION_STATE_IDLE                = 0x00000000,
    INTERACTION_STATE_IN_INTERACTION      = 0x00000001,
    INTERACTION_STATE_POSSIBLE_DOUBLE_TAP = 0x00000002,
    INTERACTION_STATE_MAX                 = 0xffffffff
} INTERACTION_STATE;

typedef enum INTERACTION_CONTEXT_PROPERTY
{
    INTERACTION_CONTEXT_PROPERTY_MEASUREMENT_UNITS       = 0x00000001,
    INTERACTION_CONTEXT_PROPERTY_INTERACTION_UI_FEEDBACK = 0x00000002,
    INTERACTION_CONTEXT_PROPERTY_FILTER_POINTERS         = 0x00000003,

    INTERACTION_CONTEXT_PROPERTY_MAX                     = 0xffffffff
} INTERACTION_CONTEXT_PROPERTY;

typedef enum CROSS_SLIDE_THRESHOLD
{
    CROSS_SLIDE_THRESHOLD_SELECT_START            = 0x00000000,
    CROSS_SLIDE_THRESHOLD_SPEED_BUMP_START        = 0x00000001,
    CROSS_SLIDE_THRESHOLD_SPEED_BUMP_END          = 0x00000002,
    CROSS_SLIDE_THRESHOLD_REARRANGE_START         = 0x00000003,

    CROSS_SLIDE_THRESHOLD_COUNT                   = 0x00000004,
    CROSS_SLIDE_THRESHOLD_MAX                     = 0xffffffff
} CROSS_SLIDE_THRESHOLD;

#define CROSS_SLIDE_THRESHOLD_INVALID_VALUE FLT_MAX

typedef enum CROSS_SLIDE_FLAGS
{
    CROSS_SLIDE_FLAGS_NONE              = 0x00000000,
    CROSS_SLIDE_FLAGS_SELECT            = 0x00000001,
    CROSS_SLIDE_FLAGS_SPEED_BUMP        = 0x00000002,
    CROSS_SLIDE_FLAGS_REARRANGE         = 0x00000004,
    CROSS_SLIDE_FLAGS_MAX               = 0xffffffff
} CROSS_SLIDE_FLAGS;

DEFINE_ENUM_FLAG_OPERATORS(CROSS_SLIDE_FLAGS);

typedef enum MOUSE_WHEEL_PARAMETER
{
    MOUSE_WHEEL_PARAMETER_CHAR_TRANSLATION_X    = 0x00000001,
    MOUSE_WHEEL_PARAMETER_CHAR_TRANSLATION_Y    = 0x00000002,
    MOUSE_WHEEL_PARAMETER_DELTA_SCALE           = 0x00000003,
    MOUSE_WHEEL_PARAMETER_DELTA_ROTATION        = 0x00000004,
    MOUSE_WHEEL_PARAMETER_PAGE_TRANSLATION_X    = 0x00000005,
    MOUSE_WHEEL_PARAMETER_PAGE_TRANSLATION_Y    = 0x00000006,
    MOUSE_WHEEL_PARAMETER_MAX                   = 0xffffffff
} MOUSE_WHEEL_PARAMETER;

#if (NTDDI_VERSION >= NTDDI_WIN10_VB)
typedef enum TAP_PARAMETER
{
    TAP_PARAMETER_MIN_CONTACT_COUNT      = 0x00000000,
    TAP_PARAMETER_MAX_CONTACT_COUNT      = 0x00000001,
    TAP_PARAMETER_MAX                    = 0xffffffff
} TAP_PARAMETER;

typedef enum HOLD_PARAMETER
{
    HOLD_PARAMETER_MIN_CONTACT_COUNT       = 0x00000000,
    HOLD_PARAMETER_MAX_CONTACT_COUNT       = 0x00000001,
    HOLD_PARAMETER_THRESHOLD_RADIUS        = 0x00000002,
    HOLD_PARAMETER_THRESHOLD_START_DELAY   = 0x00000003,
    HOLD_PARAMETER_MAX                     = 0xffffffff
} HOLD_PARAMETER;

typedef enum TRANSLATION_PARAMETER
{
    TRANSLATION_PARAMETER_MIN_CONTACT_COUNT = 0x00000000,
    TRANSLATION_PARAMETER_MAX_CONTACT_COUNT = 0x00000001,
    TRANSLATION_PARAMETER_MAX               = 0xffffffff
} TRANSLATION_PARAMETER;
#endif

typedef enum MANIPULATION_RAILS_STATE
{
    MANIPULATION_RAILS_STATE_UNDECIDED = 0x00000000,
    MANIPULATION_RAILS_STATE_FREE      = 0x00000001,
    MANIPULATION_RAILS_STATE_RAILED    = 0x00000002,
    MANIPULATION_RAILS_STATE_MAX       = 0xffffffff
} MANIPULATION_RAILS_STATE;

typedef struct MANIPULATION_TRANSFORM
{
    float translationX;
    float translationY;
    float scale;
    float expansion;
    float rotation;
} MANIPULATION_TRANSFORM;

typedef struct MANIPULATION_VELOCITY
{
    float velocityX;
    float velocityY;
    float velocityExpansion;
    float velocityAngular;
} MANIPULATION_VELOCITY;

typedef struct INTERACTION_ARGUMENTS_MANIPULATION
{
    MANIPULATION_TRANSFORM   delta;
    MANIPULATION_TRANSFORM   cumulative;
    MANIPULATION_VELOCITY    velocity;
    MANIPULATION_RAILS_STATE railsState;
} INTERACTION_ARGUMENTS_MANIPULATION;

typedef struct INTERACTION_ARGUMENTS_TAP
{
    UINT32 count;
} INTERACTION_ARGUMENTS_TAP;

typedef struct INTERACTION_ARGUMENTS_CROSS_SLIDE
{
    CROSS_SLIDE_FLAGS flags;
} INTERACTION_ARGUMENTS_CROSS_SLIDE;


typedef struct INTERACTION_CONTEXT_OUTPUT
{
    INTERACTION_ID     interactionId;
    INTERACTION_FLAGS  interactionFlags;
    POINTER_INPUT_TYPE inputType;
    float              x;
    float              y;
    union
    {
        INTERACTION_ARGUMENTS_MANIPULATION   manipulation;
        INTERACTION_ARGUMENTS_TAP            tap;
        INTERACTION_ARGUMENTS_CROSS_SLIDE    crossSlide;
    } arguments;
} INTERACTION_CONTEXT_OUTPUT;

#if (NTDDI_VERSION >= NTDDI_WIN10_VB)
typedef struct INTERACTION_CONTEXT_OUTPUT2
{
    INTERACTION_ID     interactionId;
    INTERACTION_FLAGS  interactionFlags;
    POINTER_INPUT_TYPE inputType;
    UINT32             contactCount;
    UINT32             currentContactCount;
    float              x;
    float              y;
    union
    {
        INTERACTION_ARGUMENTS_MANIPULATION   manipulation;
        INTERACTION_ARGUMENTS_TAP            tap;
        INTERACTION_ARGUMENTS_CROSS_SLIDE    crossSlide;
    } arguments;
} INTERACTION_CONTEXT_OUTPUT2;
#endif

typedef struct INTERACTION_CONTEXT_CONFIGURATION
{
    INTERACTION_ID                  interactionId;
    INTERACTION_CONFIGURATION_FLAGS enable;
} INTERACTION_CONTEXT_CONFIGURATION;

typedef struct CROSS_SLIDE_PARAMETER
{
    CROSS_SLIDE_THRESHOLD threshold;
    float distance;
} CROSS_SLIDE_PARAMETER; 


typedef void (CALLBACK *INTERACTION_CONTEXT_OUTPUT_CALLBACK) (
    _In_opt_ void *clientData,
    _In_reads_(1) const INTERACTION_CONTEXT_OUTPUT *output);

#if (NTDDI_VERSION >= NTDDI_WIN10_VB)
typedef void (CALLBACK *INTERACTION_CONTEXT_OUTPUT_CALLBACK2) (
    _In_opt_ void *clientData,
    _In_reads_(1) const INTERACTION_CONTEXT_OUTPUT2 *output);
#endif

DECLARE_HANDLE(HINTERACTIONCONTEXT);

#define INTERACTION_CONTEXT_CONFIGURATION_DEFAULT                         \
{                                                                         \
    {INTERACTION_ID_MANIPULATION,                                         \
        INTERACTION_CONFIGURATION_FLAG_MANIPULATION |                     \
        INTERACTION_CONFIGURATION_FLAG_MANIPULATION_TRANSLATION_X |       \
        INTERACTION_CONFIGURATION_FLAG_MANIPULATION_TRANSLATION_Y |       \
        INTERACTION_CONFIGURATION_FLAG_MANIPULATION_ROTATION |            \
        INTERACTION_CONFIGURATION_FLAG_MANIPULATION_SCALING |             \
        INTERACTION_CONFIGURATION_FLAG_MANIPULATION_TRANSLATION_INERTIA | \
        INTERACTION_CONFIGURATION_FLAG_MANIPULATION_ROTATION_INERTIA |    \
        INTERACTION_CONFIGURATION_FLAG_MANIPULATION_SCALING_INERTIA},     \
    {INTERACTION_ID_TAP,                                                  \
        INTERACTION_CONFIGURATION_FLAG_TAP},                              \
    {INTERACTION_ID_SECONDARY_TAP,                                        \
        INTERACTION_CONFIGURATION_FLAG_SECONDARY_TAP},                    \
    {INTERACTION_ID_HOLD,                                                 \
        INTERACTION_CONFIGURATION_FLAG_HOLD},                             \
    {INTERACTION_ID_DRAG,                                                 \
        INTERACTION_CONFIGURATION_FLAG_NONE},                             \
    {INTERACTION_ID_CROSS_SLIDE,                                          \
        INTERACTION_CONFIGURATION_FLAG_NONE}                              \
}                                                                         \

#define INERTIA_PARAMETER_INVALID_VALUE    FLT_MAX

HRESULT
WINAPI
CreateInteractionContext(
    _Out_writes_(1) HINTERACTIONCONTEXT *interactionContext);

HRESULT
WINAPI
DestroyInteractionContext(
    _In_ HINTERACTIONCONTEXT interactionContext);

HRESULT
WINAPI
RegisterOutputCallbackInteractionContext(
    _In_ HINTERACTIONCONTEXT interactionContext,
    _In_ INTERACTION_CONTEXT_OUTPUT_CALLBACK outputCallback,
    _In_opt_ void *clientData);

#if (NTDDI_VERSION >= NTDDI_WIN10_VB)
HRESULT
WINAPI
RegisterOutputCallbackInteractionContext2(
    _In_ HINTERACTIONCONTEXT interactionContext,
    _In_ INTERACTION_CONTEXT_OUTPUT_CALLBACK2 outputCallback,
    _In_opt_ void *clientData);
#endif

HRESULT
WINAPI
SetInteractionConfigurationInteractionContext(
    _In_ HINTERACTIONCONTEXT interactionContext,
    _In_ UINT32 configurationCount,
    _In_reads_(configurationCount) const INTERACTION_CONTEXT_CONFIGURATION *configuration);

HRESULT
WINAPI
GetInteractionConfigurationInteractionContext(
    _In_ HINTERACTIONCONTEXT interactionContext,
    _In_ UINT32 configurationCount,
    _Inout_updates_(configurationCount) INTERACTION_CONTEXT_CONFIGURATION *configuration);

HRESULT
WINAPI
SetPropertyInteractionContext(
    _In_ HINTERACTIONCONTEXT interactionContext,
    _In_ INTERACTION_CONTEXT_PROPERTY contextProperty,
    _In_ UINT32 value);

HRESULT
WINAPI
GetPropertyInteractionContext(
    _In_ HINTERACTIONCONTEXT interactionContext,
    _In_ INTERACTION_CONTEXT_PROPERTY contextProperty,
    _Out_writes_(1) UINT32 *value);

HRESULT
WINAPI
SetInertiaParameterInteractionContext(
    _In_ HINTERACTIONCONTEXT interactionContext,
    _In_ INERTIA_PARAMETER inertiaParameter,
    _In_ float value);

HRESULT
WINAPI
GetInertiaParameterInteractionContext(
    _In_ HINTERACTIONCONTEXT interactionContext,
    _In_ INERTIA_PARAMETER inertiaParameter,
    _Out_writes_(1) float *value);

HRESULT
WINAPI
SetCrossSlideParametersInteractionContext(
    _In_ HINTERACTIONCONTEXT interactionContext,
    _In_ UINT32 parameterCount,
    _In_reads_(parameterCount) CROSS_SLIDE_PARAMETER *crossSlideParameters);

HRESULT
WINAPI
    GetCrossSlideParameterInteractionContext(
    _In_ HINTERACTIONCONTEXT interactionContext,
    _In_ CROSS_SLIDE_THRESHOLD threshold,
    _Out_writes_(1) float *distance);

#if (NTDDI_VERSION >= NTDDI_WIN10_VB)
HRESULT
WINAPI
SetTapParameterInteractionContext(
    HINTERACTIONCONTEXT interactionContext,
    TAP_PARAMETER parameter,
    float value);

HRESULT
WINAPI
GetTapParameterInteractionContext(
    HINTERACTIONCONTEXT interactionContext,
    TAP_PARAMETER parameter,
    _Out_ float *value);

HRESULT
WINAPI
SetHoldParameterInteractionContext(
    HINTERACTIONCONTEXT interactionContext,
    HOLD_PARAMETER parameter,
    float value);

HRESULT
WINAPI
GetHoldParameterInteractionContext(
    HINTERACTIONCONTEXT interactionContext,
    HOLD_PARAMETER parameter,
    _Out_ float *value);

HRESULT
WINAPI
SetTranslationParameterInteractionContext(
    HINTERACTIONCONTEXT interactionContext,
    TRANSLATION_PARAMETER parameter,
    float value);

HRESULT
WINAPI
GetTranslationParameterInteractionContext(
    HINTERACTIONCONTEXT interactionContext,
    TRANSLATION_PARAMETER parameter,
    _Out_ float *value);
#endif

HRESULT
WINAPI
SetMouseWheelParameterInteractionContext(
    _In_ HINTERACTIONCONTEXT interactionContext,
    _In_ MOUSE_WHEEL_PARAMETER parameter,
    _In_ float value);

HRESULT
WINAPI
GetMouseWheelParameterInteractionContext(
    _In_ HINTERACTIONCONTEXT interactionContext,
    _In_ MOUSE_WHEEL_PARAMETER parameter,
    _Out_writes_(1) float *value);

HRESULT
WINAPI
ResetInteractionContext(
    _In_ HINTERACTIONCONTEXT interactionContext);

HRESULT
WINAPI
GetStateInteractionContext(
    _In_ HINTERACTIONCONTEXT interactionContext,
    _In_opt_ const POINTER_INFO *pointerInfo,
    _Out_writes_(1) INTERACTION_STATE *state);

HRESULT
WINAPI
AddPointerInteractionContext(
    _In_ HINTERACTIONCONTEXT interactionContext,
    _In_ UINT32 pointerId);

HRESULT
WINAPI
RemovePointerInteractionContext(
    _In_ HINTERACTIONCONTEXT interactionContext,
    _In_ UINT32 pointerId);

HRESULT
WINAPI
ProcessPointerFramesInteractionContext(
    _In_ HINTERACTIONCONTEXT interactionContext,
    _In_ UINT32 entriesCount,
    _In_ UINT32 pointerCount,
    _In_reads_(entriesCount * pointerCount) const POINTER_INFO *pointerInfo);

HRESULT
WINAPI
BufferPointerPacketsInteractionContext(
    _In_ HINTERACTIONCONTEXT interactionContext,
    _In_ UINT32 entriesCount,
    _In_reads_(entriesCount) const POINTER_INFO *pointerInfo);

HRESULT
WINAPI
ProcessBufferedPacketsInteractionContext(
    _In_ HINTERACTIONCONTEXT interactionContext);

HRESULT
WINAPI
ProcessInertiaInteractionContext(
    _In_ HINTERACTIONCONTEXT interactionContext);

HRESULT
WINAPI
StopInteractionContext(
    _In_ HINTERACTIONCONTEXT interactionContext);

HRESULT
WINAPI
SetPivotInteractionContext(
    _In_ HINTERACTIONCONTEXT interactionContext,
    _In_ float x,
    _In_ float y,
    _In_ float radius);

#endif /* (NTDDI_VERSION >= NTDDI_WIN8) */

#if defined(__cplusplus)
} /* __cplusplus */
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif /* INTERACTION_CONTEXT_H */

