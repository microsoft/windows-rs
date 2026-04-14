//
// Copyright (c) Microsoft Corporation.  All rights reserved.
//
//
#ifdef _MSC_VER
#pragma once
#endif

#ifndef __HRTFAPOAPI_INCLUDED__
#define __HRTFAPOAPI_INCLUDED__

#include <winapifamily.h>
#include <float.h>

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

#define HRTF_MAX_GAIN_LIMIT                  12.0f
#define HRTF_MIN_GAIN_LIMIT                  -96.0f
#define HRTF_MIN_UNITY_GAIN_DISTANCE         0.05f
#define HRTF_DEFAULT_UNITY_GAIN_DISTANCE     1.0f
#define HRTF_DEFAULT_CUTOFF_DISTANCE         FLT_MAX

//! Represents a position in 3D space, using a right-handed coordinate system.
typedef struct HrtfPosition
{
    float x;
    float y;
    float z;
} HrtfPosition;

//! Indicates the orientation of an HRTF directivity object. This is a row-major 3x3 rotation matrix.
typedef struct HrtfOrientation
{
    float element[9];
} HrtfOrientation;

//! Indicates one of several stock directivity patterns.
typedef enum HrtfDirectivityType
{
    //! The sound emission is in all directions.
    OmniDirectional=0,
    //! The sound emission is a cardiod shape.
    Cardioid,
    //! The sound emission is a cone.
    Cone
} HrtfDirectivityType;

//! Indicates one of several stock environment types.
typedef enum HrtfEnvironment
{
    //! A small room.
    Small=0,
    //! A medium-sized room.
    Medium,
    //! A large enclosed space.
    Large,
    //! An outdoor space.
    Outdoors,
} HrtfEnvironment;

//
//! Base directivity pattern descriptor. Describes the type of directivity applied to a sound.
//! The scaling parameter is used to interpolate between directivity behavior and omnidirectional; it determines how much attenuation is applied to the source outside of the directivity pattern and controls how directional the source is.
//
typedef struct HrtfDirectivity
{
    //! Indicates the type of directivity.
    HrtfDirectivityType type;
    //! A normalized value between zero and one. Specifies the amount of linear interpolation between omnidirectional sound and the full directivity pattern, where 0 is fully omnidirectional and 1 is fully directional.
    float               scaling;
} HrtfDirectivity;

//! Describes a cardioid directivity pattern.
typedef struct HrtfDirectivityCardioid
{
    //! Descriptor for the cardioid pattern. The type parameter must be set to HrtfDirectivityType.Cardioid.
    HrtfDirectivity directivity;
    //! Order controls the shape of the cardioid. The higher order the shape, the narrower it is. Must be greater than 0 and less than or equal to 32.
    float           order;
} HrtfDirectivityCardioid;

//
//! Describes a cone directivity.
//! Attenuation is 0 inside the inner cone.
//! Attenuation is linearly interpolated between the inner cone, which is defined by innerAngle, and the outer cone, which is defined by outerAngle.
//
typedef struct HrtfDirectivityCone
{
    //! Descriptor for the cone pattern. The type parameter must be set to HrtfDirectivityType.Cone.
    HrtfDirectivity directivity;
    //! Angle, in radians, that defines the inner cone. Must be between 0 and 2 * pi.
    float           innerAngle;
    //! Angle, in radians, that defines the outer cone. Must be between 0 and 2 * pi.
    float           outerAngle;
} HrtfDirectivityCone;

//
//! Indicates a distance-based decay type applied to a sound.
//
typedef enum HrtfDistanceDecayType
{
    //! Simulates natural decay with distance, as constrained by minimum and maximum gain distance limits. Drops to silence at rolloff distance.
    NaturalDecay=0,
    //! Used to set up a custom gain curve, within the maximum and minimum gain limit.
    CustomDecay
} HrtfDistanceDecayType;

//
//! Describes a distance-based decay behavior.
//
typedef struct HrtfDistanceDecay
{
    //! The type of decay behavior, natural or custom.
    HrtfDistanceDecayType type;
    //! The maximum gain limit applied at any distance. Applies to both natural and custom decay. This value is specified in dB, with a range from -96 to 12 inclusive. The default value is 12 dB.
    float                 maxGain;
    //! The minimum gain limit applied at any distance. Applies to both natural and custom decay. This value is specified in dB, with a range from -96 to 12 inclusive. The default value is -96 dB.
    float                 minGain;
    //! The distance at which the gain is 0dB. Applies to natural decay only. This value is specified in meters, with a range from 0.05 to infinity (FLT_MAX). The default value is 1 meter.
    float                 unityGainDistance;
    //! The distance at which output is silent. Applies to natural decay only. This value is specified in meters, with a range from zero (non-inclusive) to infinity (FLT_MAX). The default value is infinity.
    float                 cutoffDistance;
} HrtfDistanceDecay;

//
//! Specifies parameters used to initialize HRTF.
//! 
//! Instances of the XAPO interface are created by using the CreateHrtfApo() API:
//!   ```STDAPI CreateHrtfApo(_In_ const HrtfApoInit* pInit, _Outptr_ IXAPO** ppXapo);```
//! 
//
typedef struct HrtfApoInit
{
    //! The decay type. If you pass in nullptr, the default value will be used. The default is natural decay.
    HrtfDistanceDecay* distanceDecay;
    //! The directivity type. If you pass in nullptr, the default value will be used. The default directivity is omni-directional.
    HrtfDirectivity*   directivity;
} HrtfApoInit;

//! Creates an instance of the XAPO object.
//! Format requirements:
//! * Input: mono, 48 kHz, 32-bit float PCM.
//! * Output: stereo, 48 kHz, 32-bit float PCM.
//! Audio is processed in blocks of 1024 samples.
//! Returns:
//!     S_OK for success, any other value indicates failure.
//!     Returns E_NOTIMPL on unsupported platforms.
STDAPI CreateHrtfApo(
    //! Pointer to an HrtfApoInit struct. Specifies parameters for XAPO interface initialization.
    _In_ const HrtfApoInit* init, 
    //! Returns the new instance of the XAPO interface.
    _COM_Outptr_ IXAPO** xApo
);

//
//! The interface used to set parameters that control how HRTF is applied to a sound.
//
#undef INTERFACE
#define INTERFACE IXAPOHrtfParameters
DECLARE_INTERFACE_IID_(IXAPOHrtfParameters, IUnknown, "15B3CD66-E9DE-4464-B6E6-2BC3CF63D455")
{
    // IUnknown
    STDMETHOD(QueryInterface)(THIS_ REFIID riid, void **ppv) PURE;
    STDMETHOD_(ULONG,AddRef)(THIS) PURE;
    STDMETHOD_(ULONG,Release)(THIS) PURE;

    // HRTF params
    //! The position of the sound relative to the listener.
    STDMETHOD(SetSourcePosition)(THIS_ _In_ const HrtfPosition* position) PURE;
    //! The rotation matrix for the source orientation, with respect to the listener's frame of reference (the listener's coordinate system).
    STDMETHOD(SetSourceOrientation)(THIS_ _In_ const HrtfOrientation* orientation) PURE;
    //! The custom direct path gain value for the current source position. Valid only for sounds played with the HrtfDistanceDecayType. Custom decay type.
    STDMETHOD(SetSourceGain)(THIS_ _In_ float gain) PURE;

    // Distance cue params
    //! Selects the acoustic environment to simulate.
    STDMETHOD(SetEnvironment)(THIS_ _In_ HrtfEnvironment environment) PURE;
};

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion

#endif // __HRTFAPOAPI_INCLUDED__